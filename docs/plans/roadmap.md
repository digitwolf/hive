# Roadmap

## Phase 0 — Prepare (Sep 2026 → Mar 2027)

- [ ] Site walk: pick stand location, measure Wi-Fi, confirm power. Fill in
      `docs/apiary/site.md`.
- [ ] Zoning check + WSDA apiary registration.
- [ ] Join a local club; find a mentor and a nuc supplier. Order bees by
      January.
- [ ] Order 2× Flow Hive 2+, second brood boxes, feeders, PPE, mite gear.
- [ ] Assemble, oil, and weather the boxes over winter.
- [ ] Build the hive stand and scale platforms (`hardware/`).
- [ ] **Monitor node v0** on the bench: scale + temp/RH + brood probes +
      mic → MQTT → Grafana. Calibrate scale with known masses.
- [ ] Telemetry stack running on the home server (`apps/telemetry`).

## Phase 1 — First season (Apr 2027 → Oct 2027)

- [ ] Install bees (nucs, late April) into brood boxes on the scale
      platforms. Node v0 goes live under each hive.
- [ ] Weekly inspections logged via `/inspection`. Every log entry gets a
      timestamp so it can be lined up with the telemetry.
- [ ] Add second brood boxes; add Flow supers only if the blackberry flow
      justifies it.
- [ ] Mite counts monthly from May; treat in August.
- [ ] Node v1: weatherproofing lessons, drift compensation tuned, alerts
      (swarm, robbing, silent node).
- [ ] Winter prep: insulation + moisture quilt on both hives; baseline
      weights logged.

## Phase 2 — First winter, instrumented (Nov 2027 → Mar 2028)

- [ ] Collect cluster temp / RH / weight through the winter. Don't intervene
      except emergency feeding.
- [ ] March review: did the insulation-only approach work? Write the answer
      into `docs/research/hive-heating.md`.

## Phase 3 — Second season (2028)

- [ ] Full Flow harvest on established colonies.
- [ ] Swarm management / splits (a third hive or a nuc for insurance).
- [ ] If Phase 2 data justifies it: heater controller on **one** hive for
      winter 2028–29, hardware-limited per `hive-heating.md`.
- [ ] Entrance counter / camera as a stretch goal.

## Non-goals (for now)

- Selling honey.
- Automated Flow harvesting.
- Cloud-hosted anything; the dashboard lives on the home server.
