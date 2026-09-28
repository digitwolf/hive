//! DS18B20 probes on one 1-Wire bus: brood-nest temperatures plus the HX711
//! compensation probe (the last address in `DS18B20_ADDRS`).

use ds18b20::{Ds18b20, Resolution};
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{AnyIOPin, InputOutput, PinDriver};
use one_wire_bus::{Address, OneWire};

use crate::config::*;

pub struct BroodReading {
    /// `None` for a missing probe.
    pub probes: [Option<f32>; BROOD_PROBE_COUNT],
    pub max_c: Option<f32>,
}

pub struct Brood<'d> {
    bus: OneWire<PinDriver<'d, AnyIOPin, InputOutput>>,
}

impl<'d> Brood<'d> {
    pub fn new(pin: AnyIOPin) -> anyhow::Result<Self> {
        let pin = PinDriver::input_output_od(pin)?;
        let bus = OneWire::new(pin).map_err(|e| anyhow::anyhow!("onewire: {e:?}"))?;
        Ok(Brood { bus })
    }

    /// Print every ROM address on the bus. Used once to fill in `config.rs`.
    pub fn scan(&mut self) {
        let mut delay = Ets;
        for dev in self.bus.devices(false, &mut delay) {
            match dev {
                Ok(addr) => log::info!("1-wire device: 0x{:016X}", addr.0),
                Err(e) => log::warn!("1-wire scan error: {e:?}"),
            }
        }
    }

    fn convert_all(&mut self) {
        let mut delay = Ets;
        if ds18b20::start_simultaneous_temp_measurement(&mut self.bus, &mut delay).is_ok() {
            Resolution::Bits12.delay_for_measurement_time(&mut delay);
        }
    }

    fn read_addr(&mut self, idx: usize) -> Option<f32> {
        let addr = *DS18B20_ADDRS.get(idx)?;
        let mut delay = Ets;
        let sensor = Ds18b20::new::<()>(Address(addr)).ok()?;
        let data = sensor.read_data(&mut self.bus, &mut delay).ok()?;
        // 85.0 is the power-on default; treat it as "no conversion happened".
        if (data.temperature - 85.0).abs() < 0.01 {
            return None;
        }
        Some(data.temperature)
    }

    pub fn read(&mut self) -> BroodReading {
        self.convert_all();
        let mut probes = [None; BROOD_PROBE_COUNT];
        let mut max_c: Option<f32> = None;
        for (i, slot) in probes.iter_mut().enumerate() {
            *slot = self.read_addr(i);
            if let Some(t) = *slot {
                max_c = Some(max_c.map_or(t, |m| m.max(t)));
            }
        }
        BroodReading { probes, max_c }
    }

    /// The HX711 temperature probe (or the heater pad probe on heater builds).
    pub fn read_comp_probe(&mut self) -> Option<f32> {
        self.convert_all();
        self.read_addr(DS18B20_ADDRS.len() - 1)
    }
}
