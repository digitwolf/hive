# firmware/

| Project | Target | Purpose |
|---------|--------|---------|
| [`hive-monitor/`](hive-monitor/README.md) | ESP32 (PlatformIO, Arduino) | Per-hive sensor node: scale, temp/RH, brood probes, mic, lid switch → MQTT. Optional heater controller behind a build flag. |

Design notes: `docs/research/monitoring-sensors.md`, contract with the
server: `docs/research/telemetry-schema.md`.
