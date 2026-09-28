//! Pins, timing, calibration constants, and build-time secrets.
//! Per-hive values (hive id, scale calibration, probe addresses) are the
//! ones you'll actually edit.

/// Build-time config from `cfg.toml` (see `cfg.toml.example`).
#[toml_cfg::toml_config]
pub struct Cfg {
    #[default("hive-x")]
    hive_id: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_pass: &'static str,
    #[default("127.0.0.1")]
    mqtt_host: &'static str,
    #[default(1883)]
    mqtt_port: u16,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_pass: &'static str,
}

pub const FW_VERSION: &str = env!("CARGO_PKG_VERSION");

// ---- timing -------------------------------------------------------------
/// Deep-sleep period between samples.
pub const SAMPLE_INTERVAL_S: u64 = 300;
pub const NTP_RESYNC_S: i64 = 86_400;
pub const WIFI_TIMEOUT_MS: u32 = 15_000;
pub const MQTT_TIMEOUT_MS: u32 = 5_000;

// ---- pins (ESP32 devkit) -------------------------------------------------
// GPIO numbers; the actual pin objects are taken from `Peripherals` in main.rs
// so the compiler enforces single ownership. Listed here for the wiring doc.
//   HX711 DOUT 16, SCK 4
//   1-Wire      15
//   I2C SDA 21, SCL 22
//   I2S WS 25, SCK 26, SD 33
//   Lid reed    27  (to GND, internal pull-up; RTC-capable for EXT0 wake)
//   VBAT ADC    34  (2:1 divider; unconnected on mains builds)
//   Heater gate 32  (feature "heater")
pub const PIN_LID: i32 = 27;

// ---- scale --------------------------------------------------------------
/// 1 = print raw only, never publish. Use for calibration.
pub const SCALE_CAL_MODE: bool = false;
/// Raw reading with the empty platform.
pub const SCALE_OFFSET: i32 = 0;
/// Raw counts per kg.
pub const SCALE_SCALE: f32 = 1.0;
/// kg per °C, fitted from a temperature-swing day.
pub const SCALE_TEMP_COEF: f32 = 0.0;
pub const SCALE_TEMP_REF_C: f32 = 20.0;
pub const SCALE_SAMPLES: usize = 16;
/// Max spread across samples to call the reading "stable".
pub const SCALE_STABLE_KG: f32 = 0.05;

// ---- DS18B20 ------------------------------------------------------------
/// 1-Wire ROM addresses, from a one-time bus scan (`Bus scan` in README).
/// Index 0..BROOD_PROBE_COUNT are brood probes (published); the last entry
/// is the HX711 compensation probe (repurposed as the heater pad probe on
/// heater builds).
pub const BROOD_PROBE_COUNT: usize = 2;
pub const DS18B20_ADDRS: [u64; 3] = [
    0x0000_0000_0000_0028, // brood probe 0 (top of nest)
    0x0000_0000_0000_0028, // brood probe 1
    0x0000_0000_0000_0028, // HX711 board temperature
];

// ---- sound --------------------------------------------------------------
pub const SOUND_SAMPLE_RATE: u32 = 8_000;
/// ~8 Hz bins. Must match a `microfft::real::rfft_*` size.
pub const SOUND_FFT_N: usize = 1024;
pub const SOUND_CAPTURE_MS: u32 = 2_000;
/// Band edges in Hz; must match docs/research/telemetry-schema.md.
pub const SOUND_BANDS_HZ: [u32; 7] = [0, 100, 200, 300, 500, 1000, 2000];
pub const SOUND_BAND_COUNT: usize = SOUND_BANDS_HZ.len() - 1;

// ---- heater (feature "heater") -------------------------------------------
// The firmware setpoint is NOT the only limit: a hardware thermostat in
// series must open above HEATER_HW_CUTOFF_C. See docs/research/hive-heating.md.
pub const HEATER_SETPOINT_C: f32 = 5.0;
pub const HEATER_HYSTERESIS_C: f32 = 1.0;
/// Firmware hard-off.
pub const HEATER_MAX_PAD_C: f32 = 10.0;
/// Documented rating of the hardware thermostat (informational).
pub const HEATER_HW_CUTOFF_C: f32 = 15.0;
pub const HEATER_TICK_MS: u32 = 1_000;
/// No valid pad temperature for this long -> heater off.
pub const HEATER_PROBE_STALE_MS: u32 = 30_000;
