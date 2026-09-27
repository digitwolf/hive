# docs/research/

Research notes that feed decisions. One topic per file.

## Note format

Every note starts with a header block:

```markdown
# Title

**Status:** idea | researching | decided | superseded
**Last reviewed:** YYYY-MM-DD
**Decision:** one line, or "none yet"

## Question
## Facts (with sources)
## Local interpretation (Woodinville / Flow Hive 2+)
## Options considered
## Open questions
## Sources
```

Facts and interpretation are kept apart on purpose: a fact from a paper
about Minnesota winters is still a fact, but the interpretation for a wet
8b winter is ours and may be wrong.

## Index

| Note | Status | One-liner |
|------|--------|-----------|
| [monitoring-sensors.md](monitoring-sensors.md) | researching | ESP32 hive monitor: which sensors, which radio, how to power it |
| [hive-heating.md](hive-heating.md) | researching | Should we heat the hives in winter, and if so how, safely |
| [telemetry-schema.md](telemetry-schema.md) | decided (v0) | MQTT topics and JSON payloads the firmware and dashboard agree on |
| [open-questions.md](open-questions.md) | — | Things we don't know yet, prioritised |
