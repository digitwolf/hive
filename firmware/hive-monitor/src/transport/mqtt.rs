//! MQTT publisher. One method per topic in docs/research/telemetry-schema.md.

use std::time::Duration;

use esp_idf_svc::mqtt::client::{EspMqttClient, LwtConfiguration, MqttClientConfiguration, QoS};
use serde::Serialize;

use crate::config::{CFG, MQTT_TIMEOUT_MS};
use crate::sensors::{brood::BroodReading, climate::ClimateReading, scale::ScaleReading, sound::SoundReading};
use crate::telemetry::{self, Stamp};

pub struct Mqtt {
    client: EspMqttClient<'static>,
}

fn topic(leaf: &str) -> String {
    format!("hive/{}/{}", CFG.hive_id, leaf)
}

impl Mqtt {
    pub fn connect() -> anyhow::Result<Self> {
        let url = format!("mqtt://{}:{}", CFG.mqtt_host, CFG.mqtt_port);
        let will_topic = topic("node");
        let cfg = MqttClientConfiguration {
            client_id: Some(CFG.hive_id),
            username: (!CFG.mqtt_user.is_empty()).then_some(CFG.mqtt_user),
            password: (!CFG.mqtt_pass.is_empty()).then_some(CFG.mqtt_pass),
            lwt: Some(LwtConfiguration {
                topic: &will_topic,
                payload: br#"{"online":false}"#,
                qos: QoS::AtLeastOnce,
                retain: false,
            }),
            network_timeout: Duration::from_millis(MQTT_TIMEOUT_MS as u64),
            ..Default::default()
        };
        // Callback client; we don't subscribe, so the event handler only logs.
        let client = EspMqttClient::new_cb(&url, &cfg, |ev| {
            log::debug!("mqtt: {:?}", ev.payload());
        })?;
        Ok(Mqtt { client })
    }

    fn send<T: Serialize>(&mut self, leaf: &str, payload: &T) {
        match serde_json::to_vec(payload) {
            Ok(bytes) => {
                if let Err(e) = self.client.enqueue(&topic(leaf), QoS::AtLeastOnce, false, &bytes) {
                    log::warn!("mqtt publish {leaf}: {e}");
                }
            }
            Err(e) => log::warn!("json {leaf}: {e}"),
        }
    }

    pub fn publish_env(&mut self, r: &ClimateReading) {
        self.send("env", &telemetry::Env {
            stamp: Stamp::now(),
            t_c: telemetry::round(r.t_c),
            rh: telemetry::round(r.rh),
            t_out_c: telemetry::round(r.t_out_c),
            rh_out: telemetry::round(r.rh_out),
            p_hpa: telemetry::round(r.p_hpa),
        });
    }

    pub fn publish_weight(&mut self, r: &ScaleReading) {
        self.send("weight", &telemetry::Weight {
            stamp: Stamp::now(),
            kg: telemetry::round(Some(r.kg)).unwrap_or(f32::NAN),
            raw: r.raw,
            t_cell_c: telemetry::round(r.t_cell_c),
            stable: r.stable,
        });
    }

    pub fn publish_brood(&mut self, r: &BroodReading) {
        let mut probes = r.probes;
        for p in probes.iter_mut() {
            *p = telemetry::round(*p);
        }
        self.send("brood", &telemetry::Brood { stamp: Stamp::now(), probes, max_c: telemetry::round(r.max_c) });
    }

    pub fn publish_sound(&mut self, r: &SoundReading) {
        self.send("sound", &telemetry::Sound {
            stamp: Stamp::now(),
            rms_db: telemetry::round(Some(r.rms_db)).unwrap_or(f32::NAN),
            bands_db: r.bands_db,
            peak_hz: r.peak_hz,
        });
    }

    #[cfg(feature = "heater")]
    pub fn publish_heater(&mut self, s: &crate::heater::HeaterState) {
        self.send("heater", &telemetry::Heater {
            stamp: Stamp::now(),
            enabled: true,
            on: s.on,
            t_pad_c: telemetry::round(s.t_pad_c),
            setpoint_c: crate::config::HEATER_SETPOINT_C,
            duty_pct_1h: s.duty_pct_1h,
            fault: s.fault,
        });
    }

    pub fn publish_node(&mut self, rssi: i32, vbat: Option<f32>, reason: &'static str) {
        // SAFETY: plain ESP-IDF getter.
        let heap = unsafe { esp_idf_sys::esp_get_free_heap_size() };
        self.send("node", &telemetry::Node {
            stamp: Stamp::now(),
            online: true,
            rssi,
            vbat: telemetry::round(vbat),
            uptime_s: crate::uptime_s(),
            heap,
            reason,
        });
    }

    pub fn publish_event(&mut self, r#type: &'static str) {
        self.send("event", &telemetry::Event { stamp: Stamp::now(), r#type });
    }

    /// Give the outbox a moment to drain before Wi-Fi goes down.
    pub fn disconnect(self) {
        std::thread::sleep(Duration::from_millis(300));
        drop(self.client);
    }
}
