# telemetry stack

MQTT → Telegraf → InfluxDB 2 → Grafana, all in docker-compose. Matches the
topics/payloads in `docs/research/telemetry-schema.md`.

## Run

```sh
cp .env.example .env         # set passwords/tokens
docker compose up -d
```

- Grafana: http://localhost:3000 (admin / password from `.env`)
- InfluxDB UI: http://localhost:8086
- MQTT: port 1883 on the LAN. Firmware points at this host.

First run: in Grafana add an InfluxDB data source (Flux, org/bucket/token
from `.env`), then import `grafana/hive-overview.json`.

## Test without hardware

```sh
mosquitto_pub -h localhost -t hive/hive-a/weight \
  -m '{"ts":0,"fw":"test","kg":41.8,"raw":123456,"t_cell_c":12.0,"stable":true}'
```

The row should appear in InfluxDB under measurement `weight`, tag
`hive_id=hive-a`.

## Files

- `docker-compose.yml` — the four services, data under `./data/` (gitignored).
- `mosquitto/mosquitto.conf` — broker config (anonymous on LAN for v0;
  add a password file before exposing beyond the LAN).
- `telegraf/telegraf.conf` — MQTT consumer → InfluxDB, extracts `hive_id`
  from the topic.
- `grafana/hive-overview.json` — starter dashboard (weight, brood temp,
  RH, sound bands, node health per hive).
