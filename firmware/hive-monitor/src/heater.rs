//! Bottom-board heater controller. Compiled only with `--features heater`.
//! Design constraints: docs/research/hive-heating.md. This code is one of
//! two limits; a hardware thermostat in series with the pad is the other.
//!
//! Fail-off rules, in order, every tick:
//!   1. no valid pad temperature for HEATER_PROBE_STALE_MS -> off, fault "probe"
//!   2. pad >= HEATER_MAX_PAD_C                              -> off, fault "overtemp"
//!   3. otherwise bang-bang around HEATER_SETPOINT_C ± HEATER_HYSTERESIS_C
//! The task watchdog (sdkconfig.defaults) resets the chip if `tick` stops
//! being called; the gate pin comes up low after reset, i.e. heater off.

use std::time::Instant;

use esp_idf_hal::gpio::{AnyIOPin, Output, PinDriver};
use esp_idf_hal::task::watchdog::{TWDTConfig, TWDTDriver, WatchdogSubscription, TWDT};

use crate::config::*;

#[derive(Clone, Copy)]
pub struct HeaterState {
    pub on: bool,
    pub t_pad_c: Option<f32>,
    /// Rolling on-time over the last hour.
    pub duty_pct_1h: f32,
    pub fault: Option<&'static str>,
}

const SLOTS: usize = (3600 * 1000 / HEATER_TICK_MS) as usize;

pub struct Heater<'d> {
    gate: PinDriver<'d, AnyIOPin, Output>,
    state: HeaterState,
    last_good_probe: Option<Instant>,
    hist: Vec<bool>,
    slot: usize,
    on_count: usize,
    _wdt: WatchdogSubscription<'d>,
}

impl<'d> Heater<'d> {
    pub fn new(gate: AnyIOPin, twdt: TWDT) -> anyhow::Result<Self> {
        let mut gate = PinDriver::output(gate)?;
        gate.set_low()?;
        let cfg = TWDTConfig {
            duration: std::time::Duration::from_secs(10),
            panic_on_trigger: true,
            ..Default::default()
        };
        let mut driver = TWDTDriver::new(twdt, &cfg)?;
        let sub = driver.watch_current_task()?;
        // Leak the driver so the subscription outlives `new`; it lives for the program.
        std::mem::forget(driver);
        Ok(Heater {
            gate,
            state: HeaterState { on: false, t_pad_c: None, duty_pct_1h: 0.0, fault: None },
            last_good_probe: None,
            hist: vec![false; SLOTS],
            slot: 0,
            on_count: 0,
            _wdt: sub,
        })
    }

    fn drive(&mut self, on: bool) {
        if on {
            self.gate.set_high().ok();
        } else {
            self.gate.set_low().ok();
        }
        self.state.on = on;
    }

    fn record_duty(&mut self, on: bool) {
        if self.hist[self.slot] {
            self.on_count -= 1;
        }
        self.hist[self.slot] = on;
        if on {
            self.on_count += 1;
        }
        self.slot = (self.slot + 1) % SLOTS;
        self.state.duty_pct_1h = 100.0 * self.on_count as f32 / SLOTS as f32;
    }

    /// Call every `HEATER_TICK_MS` with the latest pad temperature reading.
    pub fn tick(&mut self, pad_c: Option<f32>) {
        self._wdt.feed().ok();

        if let Some(t) = pad_c {
            self.state.t_pad_c = Some(t);
            self.last_good_probe = Some(Instant::now());
        }

        let stale = self
            .last_good_probe
            .map_or(true, |t| t.elapsed().as_millis() > HEATER_PROBE_STALE_MS as u128);

        match self.state.t_pad_c {
            None => {
                self.state.fault = Some("probe");
                self.drive(false);
            }
            Some(_) if stale => {
                self.state.fault = Some("probe");
                self.drive(false);
            }
            Some(t) if t >= HEATER_MAX_PAD_C => {
                self.state.fault = Some("overtemp");
                self.drive(false);
            }
            Some(t) => {
                self.state.fault = None;
                if !self.state.on && t < HEATER_SETPOINT_C - HEATER_HYSTERESIS_C {
                    self.drive(true);
                } else if self.state.on && t > HEATER_SETPOINT_C + HEATER_HYSTERESIS_C {
                    self.drive(false);
                }
            }
        }
        let on = self.state.on;
        self.record_duty(on);
    }

    pub fn state(&self) -> HeaterState {
        self.state
    }
}

