# apps/

Server-side pieces that run on the home server (a small Linux box or a Pi 4
indoors — not at the hive).

| App | What |
|-----|------|
| [`telemetry/`](telemetry/README.md) | Mosquitto (MQTT broker) + Telegraf + InfluxDB 2 + Grafana via docker-compose. Receives everything the nodes publish and draws the dashboard. |

Planned:

- `alerts/` — small Python service (or Grafana alert rules) implementing
  the alert list in `docs/research/monitoring-sensors.md`.
- `beecount/` — entrance camera + counting, phase 3 stretch goal.
