---
name: inspection
description: Log a hive inspection. Walks through the inspection template for one hive, records only what the beekeeper reports, and writes a dated file to logs/inspections/.
---

Log a hive inspection.

1. Read `logs/templates/inspection.md`.
2. If the user's message already contains the observations, extract them.
   Otherwise ask, in one message, for: which hive (`hive-a`/`hive-b`),
   date/time, weather, and then the template sections in order. Accept
   partial answers; unanswered fields are written as `not checked`.
3. Write `logs/inspections/YYYY-MM-DD-<hive-id>.md` from the template.
   If the file exists, append a new `## Visit HH:MM` section instead.
4. Append a one-line row to `logs/index.md`:
   `| YYYY-MM-DD | hive-id | inspection | <10-word summary> |`.
5. Reply with the file path and any observations that usually need
   follow-up (queen cells, no eggs, mites > 2/100, stores light,
   disease signs) — as questions, not instructions.

Never invent a reading. Don't change `docs/plans/`.
