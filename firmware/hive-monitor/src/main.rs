//! hive-monitor: boot -> sample -> publish -> deep sleep.
//! Orchestration only; details live in `sensors/`, `transport/`, `heater`.

mod config;
#[cfg(feature = "heater")]
mod heater;
mod sensors;
mod telemetry;
mod transport;

use std::time::Duration;

use esp_idf_hal::adc::{attenuation, AdcChannelDriver, AdcDriver};
use esp_idf_hal::gpio::IOPin;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_sys as sys;

use config::*;
use sensors::{brood::Brood, climate::Climate, lid::Lid, scale::Scale, sound::Sound};
use transport::{mqtt::Mqtt, wifi::Net};

/// Survives deep sleep (RTC slow memory).
#[link_section = ".rtc.data"]
static mut BOOT_COUNT: u32 = 0;
#[link_section = ".rtc.data"]
static mut LAST_NTP_SYNC: i64 = 0;

fn wake_reason() -> &'static str {
    // SAFETY: plain ESP-IDF getter.
    match unsafe { sys::esp_sleep_get_wakeup_cause() } {
        sys::esp_sleep_source_t_ESP_SLEEP_WAKEUP_EXT0 => "lid",
        sys::esp_sleep_source_t_ESP_SLEEP_WAKEUP_TIMER => "timer",
        _ => "boot",
    }
}

pub fn uptime_s() -> u64 {
    // SAFETY: plain ESP-IDF getter, microseconds since boot.
    (unsafe { sys::esp_timer_get_time() } / 1_000_000) as u64
}

fn deep_sleep(lid: &Lid) -> ! {
    lid.arm_wake();
    // SAFETY: standard sleep API; never returns.
    unsafe {
        sys::esp_sleep_enable_timer_wakeup(SAMPLE_INTERVAL_S * 1_000_000);
        sys::esp_deep_sleep_start();
    }
    unreachable!()
}

struct Sensors<'d> {
    scale: Scale<'d>,
    climate: Climate<'d>,
    brood: Brood<'d>,
    sound: Sound<'d>,
    lid: Lid<'d>,
    vbat: Option<f32>,
}

/// `extra` lets a build publish additional topics (the heater state) on the
/// same connection without this function knowing about the feature.
fn sample_and_publish(s: &mut Sensors, net: &Net, reason: &'static str, extra: impl FnOnce(&mut Mqtt)) {
    let lid_open = s.lid.is_open();
    let t_cell = s.brood.read_comp_probe();
    let mut scale = s.scale.read(t_cell);
    let env = s.climate.read();
    let brood = s.brood.read();
    let sound = s.sound.capture();
    if let Some(sc) = scale.as_mut() {
        if lid_open {
            sc.stable = false; // inspection in progress
        }
    }

    let mut mqtt = match Mqtt::connect() {
        Ok(m) => m,
        Err(e) => {
            log::warn!("mqtt: {e}");
            return;
        }
    };
    mqtt.publish_env(&env);
    if let Some(sc) = &scale {
        mqtt.publish_weight(sc);
    }
    mqtt.publish_brood(&brood);
    if let Some(snd) = &sound {
        mqtt.publish_sound(snd);
    }
    extra(&mut mqtt);
    mqtt.publish_node(net.rssi(), s.vbat, reason);
    mqtt.disconnect();
}

fn main() -> anyhow::Result<()> {
    sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // SAFETY: single-threaded access at boot; these live in RTC memory.
    let boot_count = unsafe {
        BOOT_COUNT += 1;
        BOOT_COUNT
    };
    let reason = wake_reason();
    log::info!("{} fw {} boot #{} ({})", CFG.hive_id, FW_VERSION, boot_count, reason);

    let p = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // ---- sensors ----
    let i2c = I2cDriver::new(p.i2c0, p.pins.gpio21, p.pins.gpio22, &I2cConfig::new().baudrate(100.kHz().into()))?;
    let mut s = Sensors {
        scale: Scale::new(p.pins.gpio16.downgrade(), p.pins.gpio4.downgrade())?,
        climate: Climate::new(i2c),
        brood: Brood::new(p.pins.gpio15.downgrade())?,
        sound: Sound::new(p.i2s0, p.pins.gpio26, p.pins.gpio25, p.pins.gpio33)?,
        lid: Lid::new(p.pins.gpio27.downgrade())?,
        vbat: None,
    };
    // Battery through a 2:1 divider. Mains builds leave the pin floating;
    // the server ignores vbat < 2.5.
    if let Ok(adc) = AdcDriver::new(p.adc1, &esp_idf_hal::adc::config::Config::new().calibration(true)) {
        if let Ok(mut ch) = AdcChannelDriver::<{ attenuation::DB_11 }, _>::new(p.pins.gpio34) {
            if let Ok(mv) = adc.read(&mut ch) {
                s.vbat = Some(mv as f32 / 1000.0 * 2.0);
            }
        }
    }

    if reason == "boot" {
        s.brood.scan(); // one-time help for filling in DS18B20_ADDRS
    }

    if SCALE_CAL_MODE {
        loop {
            log::info!("raw={:?}", s.scale.read_raw());
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    #[cfg(feature = "heater")]
    let mut heater = heater::Heater::new(p.pins.gpio32.downgrade(), p.twdt)?;

    // ---- network ----
    let net = match Net::connect(p.modem, sysloop, nvs) {
        Ok(n) => Some(n),
        Err(e) => {
            log::warn!("{e}");
            None
        }
    };

    if let Some(net) = &net {
        // SAFETY: RTC statics, single-threaded here.
        let last = unsafe { LAST_NTP_SYNC };
        let now = transport::wifi::epoch_now().unwrap_or(0);
        if last == 0 || now - last > NTP_RESYNC_S {
            if net.ntp_sync() {
                unsafe { LAST_NTP_SYNC = transport::wifi::epoch_now().unwrap_or(0) };
            }
        }
        if reason != "timer" {
            if let Ok(mut m) = Mqtt::connect() {
                m.publish_event(match reason {
                    "lid" if s.lid.is_open() => "lid_open",
                    "lid" => "lid_closed",
                    _ => "boot",
                });
                m.disconnect();
            }
        }
        #[cfg(not(feature = "heater"))]
        sample_and_publish(&mut s, net, reason, |_| {});
        #[cfg(feature = "heater")]
        sample_and_publish(&mut s, net, reason, |m| m.publish_heater(&heater.state()));
    }

    #[cfg(not(feature = "heater"))]
    {
        s.scale.power_down();
        if let Some(n) = net {
            n.disconnect();
        }
        log::info!("sleep {SAMPLE_INTERVAL_S} s");
        deep_sleep(&s.lid);
    }

    // Heater builds stay awake: control tick every second, publish on schedule.
    #[cfg(feature = "heater")]
    {
        let mut net = net;
        let mut last_sample = std::time::Instant::now();
        loop {
            let pad = s.brood.read_comp_probe();
            heater.tick(pad);
            if last_sample.elapsed() >= Duration::from_secs(SAMPLE_INTERVAL_S) {
                last_sample = std::time::Instant::now();
                if let Some(n) = net.as_mut() {
                    if n.ensure_connected() {
                        sample_and_publish(&mut s, n, "timer", |m| m.publish_heater(&heater.state()));
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(HEATER_TICK_MS as u64));
        }
    }
}
