# CLAUDE.md

Project: a two-hive Flow Hive 2+ apiary in Woodinville, WA, plus the
monitoring/automation built around it. This file tells Claude how to work here.

## What this repo is

- `docs/` is the knowledge base. Markdown only. Every research note has a
  header block (status, last-reviewed date, sources) — see
  `docs/research/README.md`.
- `logs/` is the record of what actually happened to the bees. Never invent
  entries; only write from what the user reports.
- `firmware/hive-monitor/` is a Rust (`esp-idf-svc`) ESP32 project. **All
  firmware in this repo is written in Rust** — no Arduino/C++.
- `apps/telemetry/` is the home-server side (MQTT broker, InfluxDB, Grafana).
- `hardware/` holds the bill of materials and wiring notes.
- `.claude/agents/` and `.claude/skills/` define the Claude helpers.

## Conventions

- Dates are ISO (`2027-04-18`). Filenames for dated records are
  `YYYY-MM-DD-<slug>.md`.
- Hives are named `hive-a` and `hive-b` (physical labels on the boxes match).
- Units: °C for temperature in firmware/data, °F may be shown alongside in
  docs. Weight in kg. Humidity % RH.
- Research notes must separate **facts with sources**, **local
  interpretation** (what it means for Woodinville / Flow Hive), and
  **open questions**. Flag uncertainty; don't smooth it over.
- Beekeeping advice is location-dependent. Default context: USDA 8b, wet mild
  winters, blackberry is the main summer flow, varroa is the main threat.
- Safety notes on anything involving heaters, mains power, or lithium
  batteries are mandatory, not optional.

## Firmware

- Language: Rust. Target: ESP32 (Xtensa, `xtensa-esp32-espidf`), std on
  ESP-IDF via `esp-idf-svc` / `esp-idf-hal`. Toolchain: `espup` (`esp`
  channel), `ldproxy`, `espflash`.
- Don't add a crate to `Cargo.toml` without a one-line comment above it
  saying why.
- Sensor reads happen in `src/sensors/`, transport in `src/transport/`,
  payload structs in `src/telemetry.rs`, glue in `src/main.rs`. Keep
  `main.rs` small. `unsafe` only for direct `esp_idf_sys` calls, each with
  a `// SAFETY:` line.
- Publish to MQTT topic `hive/<hive-id>/<metric>`; payloads are JSON. Topic
  and payload schema are documented in `docs/research/telemetry-schema.md`
  and mirrored by the serde structs in `src/telemetry.rs` — update all
  together.
- Optional hardware (heater) goes behind a cargo feature, default off.
- There is no hardware in CI. Say clearly in any PR/commit whether the code
  was compiled (`cargo build`) and whether it was tested on a device.

## Git

- Small, descriptive commits. Research notes and firmware go in separate
  commits.
- Don't rewrite history on `main`.
