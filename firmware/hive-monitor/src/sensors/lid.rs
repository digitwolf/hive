//! Roof reed switch. Closed (magnet present, lid on) pulls the pin LOW.
//! Also the EXT0 deep-sleep wake source so an inspection is logged at once.

use esp_idf_hal::gpio::{AnyIOPin, Input, PinDriver, Pull};
use esp_idf_sys as sys;

use crate::config::PIN_LID;

pub struct Lid<'d> {
    pin: PinDriver<'d, AnyIOPin, Input>,
}

impl<'d> Lid<'d> {
    pub fn new(pin: AnyIOPin) -> anyhow::Result<Self> {
        let mut pin = PinDriver::input(pin)?;
        pin.set_pull(Pull::Up)?;
        Ok(Lid { pin })
    }

    pub fn is_open(&self) -> bool {
        self.pin.is_high()
    }

    /// Wake on the opposite level from the current one, so either edge wakes us.
    pub fn arm_wake(&self) {
        let level = if self.is_open() { 0 } else { 1 };
        // SAFETY: plain ESP-IDF call with a valid RTC GPIO number.
        unsafe {
            sys::esp_sleep_enable_ext0_wakeup(PIN_LID, level);
        }
    }
}
