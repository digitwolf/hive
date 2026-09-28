//! Sensor drivers. Each exposes a small `Reading` struct; conversion to the
//! wire format happens in `transport::mqtt`.
pub mod brood;
pub mod climate;
pub mod lid;
pub mod scale;
pub mod sound;
