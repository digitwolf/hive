//! HX711 load-cell amplifier, bit-banged (the protocol is 25 clock pulses;
//! a crate isn't worth the dependency). Channel A, gain 128.

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{AnyIOPin, Input, Output, PinDriver};
use esp_idf_svc::hal::task::CriticalSection;

use crate::config::*;

pub struct ScaleReading {
    /// Tared, temperature-compensated.
    pub kg: f32,
    /// Averaged raw counts, for server-side recalibration.
    pub raw: i32,
    /// HX711 board temperature used for compensation.
    pub t_cell_c: Option<f32>,
    /// Sample spread below `SCALE_STABLE_KG`.
    pub stable: bool,
}

pub struct Scale<'d> {
    dout: PinDriver<'d, AnyIOPin, Input>,
    sck: PinDriver<'d, AnyIOPin, Output>,
    cs: CriticalSection,
}

impl<'d> Scale<'d> {
    pub fn new(dout: AnyIOPin, sck: AnyIOPin) -> anyhow::Result<Self> {
        let dout = PinDriver::input(dout)?;
        let mut sck = PinDriver::output(sck)?;
        sck.set_low()?; // SCK low = powered up
        Ok(Scale { dout, sck, cs: CriticalSection::new() })
    }

    fn wait_ready(&self, timeout_ms: u32) -> bool {
        for _ in 0..timeout_ms {
            if self.dout.is_low() {
                return true;
            }
            Ets::delay_ms(1);
        }
        false
    }

    /// One 24-bit two's-complement sample. SCK must stay high < 60 µs or
    /// the chip powers down, hence the critical section.
    fn read_once(&mut self) -> Option<i32> {
        if !self.wait_ready(1000) {
            return None;
        }
        let mut v: u32 = 0;
        {
            let _guard = self.cs.enter();
            for _ in 0..24 {
                self.sck.set_high().ok();
                Ets::delay_us(1);
                v = (v << 1) | (self.dout.is_high() as u32);
                self.sck.set_low().ok();
                Ets::delay_us(1);
            }
            // 25th pulse: next conversion channel A, gain 128.
            self.sck.set_high().ok();
            Ets::delay_us(1);
            self.sck.set_low().ok();
        }
        // Sign-extend 24 -> 32 bits.
        Some(((v << 8) as i32) >> 8)
    }

    pub fn read_raw(&mut self) -> Option<i32> {
        self.read_once()
    }

    pub fn read(&mut self, t_cell_c: Option<f32>) -> Option<ScaleReading> {
        let mut sum: i64 = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for _ in 0..SCALE_SAMPLES {
            let v = self.read_once()?;
            sum += v as i64;
            min = min.min(v);
            max = max.max(v);
        }
        let raw = (sum / SCALE_SAMPLES as i64) as i32;
        let mut kg = (raw - SCALE_OFFSET) as f32 / SCALE_SCALE;
        if let Some(t) = t_cell_c {
            kg -= SCALE_TEMP_COEF * (t - SCALE_TEMP_REF_C);
        }
        let stable = ((max - min) as f32 / SCALE_SCALE) < SCALE_STABLE_KG;
        Some(ScaleReading { kg, raw, t_cell_c, stable })
    }

    /// SCK held high powers the HX711 down (~1.5 mA saved during deep sleep).
    pub fn power_down(&mut self) {
        self.sck.set_high().ok();
    }
}
