//! In-hive SHT31 (under the inner cover) + outside BME280, on one I2C bus.
//! SHT31 single-shot is three lines of protocol, so it's inline; BME280 has
//! calibration maths and uses the `bme280` crate.

use std::cell::RefCell;

use bme280::i2c::BME280;
use embedded_hal::i2c::I2c;
use embedded_hal_bus::i2c::RefCellDevice;
use esp_idf_hal::delay::{Delay, Ets};
use esp_idf_hal::i2c::I2cDriver;

pub struct ClimateReading {
    /// Under the inner cover.
    pub t_c: Option<f32>,
    pub rh: Option<f32>,
    /// Outside, if the BME280 is fitted.
    pub t_out_c: Option<f32>,
    pub rh_out: Option<f32>,
    pub p_hpa: Option<f32>,
}

const SHT31_ADDR: u8 = 0x44;

pub struct Climate<'d> {
    bus: RefCell<I2cDriver<'d>>,
    bme_ok: bool,
}

impl<'d> Climate<'d> {
    pub fn new(i2c: I2cDriver<'d>) -> Self {
        Climate { bus: RefCell::new(i2c), bme_ok: false }
    }

    fn read_sht31(&self) -> Option<(f32, f32)> {
        let mut dev = RefCellDevice::new(&self.bus);
        // Single shot, high repeatability, no clock stretching.
        dev.write(SHT31_ADDR, &[0x24, 0x00]).ok()?;
        Ets::delay_ms(20);
        let mut buf = [0u8; 6];
        dev.read(SHT31_ADDR, &mut buf).ok()?;
        if crc8(&buf[0..2]) != buf[2] || crc8(&buf[3..5]) != buf[5] {
            return None;
        }
        let t_raw = u16::from_be_bytes([buf[0], buf[1]]) as f32;
        let h_raw = u16::from_be_bytes([buf[3], buf[4]]) as f32;
        let t = -45.0 + 175.0 * t_raw / 65535.0;
        let h = 100.0 * h_raw / 65535.0;
        Some((t, h))
    }

    fn read_bme280(&mut self) -> Option<(f32, f32, f32)> {
        let mut delay = Delay::new_default();
        // Try both common addresses; init each time — we're awake for seconds only.
        for addr_primary in [true, false] {
            let mut bme = if addr_primary {
                BME280::new_primary(RefCellDevice::new(&self.bus))
            } else {
                BME280::new_secondary(RefCellDevice::new(&self.bus))
            };
            if bme.init(&mut delay).is_ok() {
                if let Ok(m) = bme.measure(&mut delay) {
                    self.bme_ok = true;
                    return Some((m.temperature, m.humidity, m.pressure / 100.0));
                }
            }
        }
        None
    }

    pub fn read(&mut self) -> ClimateReading {
        let (t_c, rh) = match self.read_sht31() {
            Some((t, h)) => (Some(t), Some(h)),
            None => (None, None),
        };
        let (t_out_c, rh_out, p_hpa) = match self.read_bme280() {
            Some((t, h, p)) => (Some(t), Some(h), Some(p)),
            None => (None, None, None),
        };
        ClimateReading { t_c, rh, t_out_c, rh_out, p_hpa }
    }
}

/// SHT3x CRC-8: poly 0x31, init 0xFF.
fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xFF;
    for &b in data {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x31 } else { crc << 1 };
        }
    }
    crc
}
