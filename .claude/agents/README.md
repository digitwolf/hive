# Claude agents for this repo

Agent definitions Claude Code picks up automatically (frontmatter: `name`,
`description`, `tools`). Invoke by asking for the task, e.g. "research
whether a moisture quilt fits a Flow Hive roof" → `apiary-researcher`.

| Agent | Job | Writes to |
|-------|-----|-----------|
| `beekeeper` | Expert local mentor: hands-on beekeeping in Woodinville, colony management, seasonal tasks, and everything varroa (counts, thresholds, treatments, timing). Advises; doesn't write files. | — |
| `apiary-researcher` | Investigates a beekeeping / sensor / heating question and writes a research note in the standard format, with sources and a clear facts-vs-interpretation split. | `docs/research/` |
| `firmware-engineer` | Implements and reviews ESP32 firmware for the hive monitor; keeps `telemetry-schema.md` and Telegraf config in sync. | `firmware/`, `apps/telemetry/`, `docs/research/telemetry-schema.md` |
| `inspection-scribe` | Turns a spoken/typed account of a hive inspection into a structured log entry. Never invents observations. | `logs/` |
| `season-planner` | Reads the calendar, the plan, and recent logs; proposes the next 4–6 weeks of tasks and updates the season plan. | `docs/plans/` |

Skills (`.claude/skills/`):

| Skill | Use |
|-------|-----|
| `/inspection` | Log a hive inspection interactively. |
