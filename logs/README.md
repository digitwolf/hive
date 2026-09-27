# logs/

The record of what actually happened. Only written from the beekeeper's own
reports (via the `/inspection` skill or the `inspection-scribe` agent).

```
logs/
├── index.md              # one line per entry, newest last
├── inspections/          # YYYY-MM-DD-<hive-id>.md
├── treatments/           # mite counts and treatments
├── feeding/              # what, how much, when
├── harvests/             # Flow frame harvests
└── templates/            # blank forms
```

Timestamps matter: telemetry in Grafana can be lined up against an
inspection if the log says `14:35` rather than "afternoon". Lid-open events
from the node also mark inspections automatically once v1 ships.
