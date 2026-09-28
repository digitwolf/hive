//! Payload types — the Rust side of docs/research/telemetry-schema.md.
//! Field names are the wire names. Optional floats serialise as absent when
//! `None`, matching "optional key" in the schema.

use serde::Serialize;

use crate::config::{BROOD_PROBE_COUNT, FW_VERSION, SOUND_BAND_COUNT};

fn r2(v: f32) -> f32 {
    (v * 100.0).round() / 100.0
}

/// `ts` (UTC epoch seconds, 0 if unsynced) + `fw`, present on every message.
#[derive(Serialize)]
pub struct Stamp {
    pub ts: i64,
    pub fw: &'static str,
}

impl Stamp {
    pub fn now() -> Self {
        let ts = crate::transport::wifi::epoch_now().unwrap_or(0);
        Stamp { ts, fw: FW_VERSION }
    }
}

#[derive(Serialize)]
pub struct Env {
    #[serde(flatten)]
    pub stamp: Stamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_c: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rh: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_out_c: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rh_out: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_hpa: Option<f32>,
}

#[derive(Serialize)]
pub struct Weight {
    #[serde(flatten)]
    pub stamp: Stamp,
    pub kg: f32,
    pub raw: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_cell_c: Option<f32>,
    pub stable: bool,
}

#[derive(Serialize)]
pub struct Brood {
    #[serde(flatten)]
    pub stamp: Stamp,
    /// `null` for a missing probe, ordered as `config::DS18B20_ADDRS`.
    pub probes: [Option<f32>; BROOD_PROBE_COUNT],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_c: Option<f32>,
}

#[derive(Serialize)]
pub struct Sound {
    #[serde(flatten)]
    pub stamp: Stamp,
    pub rms_db: f32,
    pub bands_db: [f32; SOUND_BAND_COUNT],
    pub peak_hz: u32,
}

#[derive(Serialize)]
pub struct Heater {
    #[serde(flatten)]
    pub stamp: Stamp,
    pub enabled: bool,
    pub on: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_pad_c: Option<f32>,
    pub setpoint_c: f32,
    pub duty_pct_1h: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault: Option<&'static str>,
}

#[derive(Serialize)]
pub struct Node {
    #[serde(flatten)]
    pub stamp: Stamp,
    pub online: bool,
    pub rssi: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbat: Option<f32>,
    pub uptime_s: u64,
    pub heap: u32,
    pub reason: &'static str,
}

#[derive(Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub stamp: Stamp,
    pub r#type: &'static str,
}

/// Round to 2 dp for the wire; keeps payloads short.
pub fn round(v: Option<f32>) -> Option<f32> {
    v.filter(|x| x.is_finite()).map(r2)
}
