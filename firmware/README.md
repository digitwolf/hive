# firmware/

All firmware in this repo is **Rust**.

| Project | Target | Purpose |
|---------|--------|---------|
| [`hive-monitor/`](hive-monitor/README.md) | ESP32 (Rust, `esp-idf-svc`) | Per-hive sensor node: scale, temp/RH, brood probes, mic, lid switch → MQTT. Optional heater controller behind a cargo feature. |

Design notes: `docs/research/monitoring-sensors.md`; contract with the
server: `docs/research/telemetry-schema.md`.
