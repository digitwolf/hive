# Hive

Home base for setting up, running, and improving a two-hive apiary of
**Flow Hive 2+** colonies in **Woodinville, WA** (USDA zone 8b, Puget Sound
lowlands).

The repo holds three kinds of things:

| Area | What lives there | Start here |
|------|------------------|------------|
| **Knowledge** | Site notes, equipment, seasonal calendar, research write-ups | [`docs/`](docs/README.md) |
| **Agents** | Claude Code agent definitions and skills that do research, keep logs, and write firmware | [`.claude/agents/`](.claude/agents/README.md) |
| **Automation** | ESP32 hive-monitor firmware, telemetry stack, heater controller design | [`firmware/`](firmware/README.md), [`apps/`](apps/README.md) |
| **Records** | Inspection logs, treatments, harvests | [`logs/`](logs/README.md) |

## Goals

1. **Establish** two healthy colonies in Flow Hive 2+ equipment (first bees spring 2027).
2. **Monitor remotely**: hive weight, in-hive temperature/humidity, sound, and
   outside weather from a low-power ESP32 node per hive, pushed to a home
   dashboard. See [`docs/research/monitoring-sensors.md`](docs/research/monitoring-sensors.md).
3. **Keep the cluster warm in the wet PNW winter**: insulation first, then an
   optional thermostatically-limited heater. See
   [`docs/research/hive-heating.md`](docs/research/hive-heating.md).
4. **Keep good records** so decisions (feeding, mite treatment, harvest) are
   based on data, not guesswork.

## Layout

```
.
├── .claude/
│   ├── agents/        # Claude agent definitions (researcher, firmware, scribe, planner)
│   └── skills/        # Repo skills (e.g. /inspection to log a hive check)
├── docs/
│   ├── apiary/        # Site, equipment, seasonal calendar, regulations
│   ├── research/      # Research notes: sensors, heating, open questions
│   └── plans/         # Roadmap and season plans
├── firmware/
│   └── hive-monitor/  # PlatformIO project for the ESP32 node
├── apps/
│   └── telemetry/     # MQTT + InfluxDB + Grafana stack for the home server
├── hardware/          # BOM, wiring notes, enclosure notes
└── logs/              # Inspection / treatment / harvest records + templates
```

## Working with Claude in this repo

`CLAUDE.md` describes conventions. Useful entry points:

- "Research X" → the `apiary-researcher` agent writes a note into `docs/research/`.
- "Log today's inspection" → `/inspection` skill creates a dated file in `logs/inspections/`.
- "Plan next month" → `season-planner` agent reads the calendar + logs and updates `docs/plans/`.
- Firmware changes → `firmware-engineer` agent works inside `firmware/hive-monitor`.

## Status

- [x] Repo scaffolded (Sept 2026)
- [ ] Site chosen and prepared (stand, wind/rain shelter, water source)
- [ ] WSDA apiary registration
- [ ] Equipment assembled and painted/oiled
- [ ] Bees ordered (nuc or package) for April/May 2027
- [ ] Monitor node v0 on the bench
- [ ] Monitor node v1 under a hive
- [ ] Winter insulation plan tested (2027–28)
