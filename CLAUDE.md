# CLAUDE.md

Project: a two-hive Flow Hive 2+ apiary in Woodinville, WA, plus the
monitoring/automation built around it. This file tells Claude how to work here.

## What this repo is

- `docs/` is the knowledge base. Markdown only. Every research note has a
  header block (status, last-reviewed date, sources) — see
  `docs/research/README.md`.
- `logs/` is the record of what actually happened to the bees. Never invent
  entries; only write from what the user reports.
- `firmware/hive-monitor/` is a PlatformIO ESP32 project.
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

- Target board: ESP32 (`esp32dev`), PlatformIO, Arduino framework.
- Don't add a library to `platformio.ini` without a one-line reason in the
  commit message.
- Sensor reads happen in `src/sensors/`, transport in `src/transport/`,
  glue in `src/main.cpp`. Keep `main.cpp` small.
- Publish to MQTT topic `hive/<hive-id>/<metric>`; payloads are JSON. Topic
  and payload schema are documented in `docs/research/telemetry-schema.md`
  — update both together.
- There is no hardware in CI. Say clearly in any PR/commit whether the code
  was compiled (`pio run`) and whether it was tested on a device.

## Git

- Small, descriptive commits. Research notes and firmware go in separate
  commits.
- Don't rewrite history on `main`.
