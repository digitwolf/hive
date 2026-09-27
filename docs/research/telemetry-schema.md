# Telemetry schema (v0)

**Status:** decided (v0)
**Last reviewed:** 2026-09-27
**Decision:** MQTT, one topic per metric group, JSON payloads, UTC epoch seconds. Firmware (`firmware/hive-monitor`) and the Telegraf config (`apps/telemetry`) must both match this file.

## Topics

```
hive/<hive_id>/env        in-hive environment
hive/<hive_id>/weight     scale
hive/<hive_id>/brood      brood-nest temperature probes
hive/<hive_id>/sound      audio features
hive/<hive_id>/heater     heater state (only when HEATER_ENABLED)
hive/<hive_id>/node       node health
hive/<hive_id>/event      discrete events (lid open, tilt, boot)
```

`hive_id` ∈ `hive-a`, `hive-b`. All messages retained=false, QoS 1.
Node publishes `hive/<hive_id>/node` with `"online": false` as its LWT.

## Payloads

All payloads carry `"ts"` (UTC epoch seconds, from NTP; `0` if not synced —
the server substitutes receive time) and `"fw"` (firmware version string).

### `env`
```json
{"ts": 1730000000, "fw": "0.1.0", "t_c": 12.4, "rh": 78.2, "t_out_c": 8.1, "rh_out": 91.0, "p_hpa": 1012.3}
```
`t_c`/`rh` are under the inner cover. `t_out_c`/`rh_out`/`p_hpa` optional (outside BME280).

### `weight`
```json
{"ts": 1730000000, "fw": "0.1.0", "kg": 41.83, "raw": 1234567, "t_cell_c": 11.9, "stable": true}
```
`kg` is temperature-compensated and tared. `raw` is the averaged HX711 reading for re-calibration server-side. `stable` false while readings are noisy (wind, inspection).

### `brood`
```json
{"ts": 1730000000, "fw": "0.1.0", "probes": [34.7, 33.9, 21.2], "max_c": 34.7}
```
`probes` ordered by DS18B20 ROM address list in `config.h`.

### `sound`
```json
{"ts": 1730000000, "fw": "0.1.0", "rms_db": 52.3, "bands_db": [40.1, 47.8, 51.2, 44.0, 38.7, 30.2], "peak_hz": 245}
```
`bands_db`: energy in 0–100, 100–200, 200–300, 300–500, 500–1000, 1000–2000 Hz. Sample: 2 s @ 8 kHz.

### `heater`
```json
{"ts": 1730000000, "fw": "0.1.0", "enabled": true, "on": false, "t_pad_c": 6.8, "setpoint_c": 5.0, "duty_pct_1h": 12.5}
```

### `node`
```json
{"ts": 1730000000, "fw": "0.1.0", "online": true, "rssi": -68, "vbat": 4.02, "uptime_s": 86400, "heap": 180000, "reason": "timer"}
```

### `event`
```json
{"ts": 1730000000, "fw": "0.1.0", "type": "lid_open"}
```
`type` ∈ `boot`, `lid_open`, `lid_closed`, `tilt`, `ota_start`, `ota_ok`, `heater_fault`.

## Storage

Telegraf → InfluxDB 2.x bucket `hive`, measurement = topic leaf (`env`,
`weight`, …), tag `hive_id`, fields = JSON numeric keys, arrays flattened
(`probes_0`, `bands_db_2`, …).

## Versioning

Additive changes (new optional keys) don't bump the schema. Renames or unit
changes bump `fw` minor version and get a dated note here.
