# Remote hive monitoring with an ESP32

**Status:** researching
**Last reviewed:** 2026-09-27
**Decision:** none yet — v0 bench build with ESP32 + HX711 load cells + SHT31 + DS18B20 + INMP441 mic over Wi-Fi/MQTT; revisit radio and power after measuring signal and winter sun at the actual site.

## Question

Can we tell how each colony is doing without opening the hive, from the
house, and get alerted when something changes (swarm, robbing, starvation,
dead-out, hive knocked over)?

## What the signals mean

| Signal | What it tells you | Sensor | Sample rate |
|--------|-------------------|--------|-------------|
| **Weight** | Stores, nectar flow, consumption rate, swarm (sharp −1–2 kg), robbing (steady fast loss), super full, hive tipped | 4× 50 kg half-bridge load cells + HX711, or 1× single-point 100–200 kg cell | 5–15 min; the daily curve is the information |
| **Brood-nest temperature** | Queen laying (34–36 °C held tight), broodless (cluster 20–30 °C, drifts), dead-out (tracks ambient) | DS18B20 probe(s) pushed into the top of the brood nest; or a "temperature bar" of several DS18B20 across frames | 5 min |
| **In-hive humidity** | Condensation risk in winter (>80–85 % RH at the top = wet bees), ventilation | SHT31/SHT40 or BME280 under the inner cover, *not* in the cluster (sensor drift) | 5 min |
| **Sound** | Queenlessness (higher-pitched, "roaring"), pre-swarm activity, piping; very rough activity level | I2S MEMS mic (INMP441 / SPH0645) at the inner cover; compute band energy on-device, publish features not audio | 1 min bursts |
| **Outside weather** | Context for all of the above; foraging days | BME280 in a radiation shield; or pull from a nearby weather API on the server | 5 min |
| **Entrance activity** | Foraging strength, orientation flights, robbing | Optional: IR beam counters, or camera + counting on the server. Skip for v0. | — |
| **Lid open / tilt** | Inspection in progress (suppress alerts), hive knocked over (bear) | Reed switch on roof; MPU6050 or just the load-cell pattern | event |

Weight is the highest-value signal per dollar. If only one thing gets built,
build the scale.

## Facts (with sources)

- Commercial and open hive scales converge on 4× 50 kg half-bridge cells
  under a platform, HX711 at 10 SPS, averaged. Accuracy ±50–100 g is
  achievable after temperature compensation; raw HX711 drift with
  temperature is the main error source (cells + HX711 both drift; a
  DS18B20 next to the HX711 and a linear correction handles most of it).
  (Open sources: BeeHive-Monitoring projects on GitHub, HiveEyes /
  Open Hive scale docs, Arnia / BroodMinder product docs for the
  what-signals-matter part.)
- Brood nest is held at 34.5–35.5 °C when brood is present; winter cluster
  core is 20–35 °C depending on ambient, cluster mantle down to ~10 °C.
  (Seeley, *The Lives of Bees*; Southwick's cluster thermoregulation work.)
- Acoustic monitoring: queenless colonies show increased energy in the
  ~200–300 Hz and above-500 Hz bands; the literature works, but each hive
  needs its own baseline. Do features on-device (FFT band energies) and log
  them; don't try to classify in year one. (Ramsey et al. 2020, "The
  prediction of swarming in honeybee colonies using vibrational spectra";
  various "bee sound" datasets on Kaggle/Zenodo for offline experiments.)
- ESP32 deep-sleep current ~10 µA for the bare chip; a real dev board with
  its USB-UART and LDO is 1–10 mA and that ruins battery life. Use a board
  without them (e.g. ESP32-WROOM on a bare carrier, FireBeetle, or an
  ESP32-C3/S3 mini with the LDO bypassed) or accept mains power.
- Wi-Fi association + MQTT publish burst is ~150–250 mA for 1–3 s. At a 10
  min interval that's roughly 5–10 mAh/day → a 3000 mAh 18650 lasts
  months *if* the sleep current is actually low. LoRa (SX1276) publish is
  cheaper and reaches further but needs a gateway.

## Local interpretation

- Winter solar in Woodinville is not reliable. If mains is within ~30 m of
  the stand, **run mains** (12 V outdoor-rated supply, buck to 5 V) and stop
  worrying about power. Mains also makes the heater question (see
  `hive-heating.md`) tractable. Battery+solar is a fallback, not the plan.
- Wet, cold, condensing air for five months: every board goes in an IP65+
  box with a Gore vent, connectors get dielectric grease, the SHT31 needs a
  PTFE membrane cap or it will read 100 % and die.
- Two hives ~1 m apart: one ESP32 per hive is simpler than a shared node
  with long analog runs (HX711 lines are noisy over >1 m). Two nodes, same
  firmware, different `HIVE_ID`.
- Radio: measure Wi-Fi at the stand first. If ≥ −70 dBm, plain Wi-Fi.
  Otherwise an outdoor AP is cheaper and simpler than a LoRa gateway.

## Options considered

### Radio

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| Wi-Fi + MQTT | Simplest, no extra hardware, ESP32 native | Power-hungry, range | **v0** |
| ESP-NOW to a relay ESP32 near the house | Very low power, no AP association | Custom relay, still Wi-Fi at the house end | v1 if battery |
| LoRa → gateway (or LoRaWAN / TTN) | km range, tiny power | Gateway cost, payload limits (no audio features) | Only if Wi-Fi impossible |
| Zigbee/Thread | Fits Home Assistant | Range, fewer sensor libs | No |

### Compute / platform

- **ESP32 + Rust (`esp-idf-svc`, std on ESP-IDF)**: Wi-Fi/MQTT/SNTP/I2S/
  deep sleep all wrapped; `bme280`, `ds18b20`/`one-wire-bus`, `microfft`
  crates cover the sensors; HX711 and SHT31 are a few lines of protocol
  each. **Chosen** — all firmware in this repo is Rust. Trade-off vs
  bare-metal `esp-hal`: bigger binary and slower boot, which doesn't matter
  for a node that wakes every 5 minutes.
- ESP32 + Arduino/PlatformIO: the largest sensor-library ecosystem, but
  ruled out by the all-Rust decision.
- ESPHome: fast to stand up, great with Home Assistant, but on-device FFT
  and custom drift compensation are awkward, and it isn't Rust.
- Raspberry Pi Zero: full Linux, easy camera, but power and SD-card rot in
  the cold. No.

### Off-the-shelf comparison (to sanity-check the build)

BroodMinder (BLE, cheap, needs a hub), Arnia, HiveTracks, Beep. A
BroodMinder-W scale is ~$200; our BOM per hive should come in under that
with more sensors, or the DIY isn't worth it. See `../../hardware/BOM.md`.

## Alerts worth having (server side, v1)

- Weight drop > 1 kg in < 1 h during daylight, Apr–Jul → possible swarm.
- Weight loss > 0.5 kg/day sustained in Aug–Sep → robbing.
- Brood temp < 30 °C while ambient < 10 °C for > 12 h in winter → cluster
  gone or queen dead (compare to the other hive).
- Top-box RH > 85 % for > 6 h → condensation, check quilt.
- Node silent > 1 h → power/Wi-Fi.

## Open questions

- [ ] Actual Wi-Fi RSSI at the hive stand.
- [ ] Mains available at the stand? (Decides power *and* heater feasibility.)
- [ ] Load-cell platform design that works with Flow Hive 2+ legs — remove
      legs and sit the whole hive on the platform, or build the platform
      with four cups the legs stand in?
- [ ] How much HX711 drift do we see in a PNW temperature swing (−5 to 35 °C)?
      Bench test: freezer → room → hot car with a known mass.
- [ ] Is an on-device FFT worth it in v0, or log raw RMS first?

## Sources

- Seeley, T. *The Lives of Bees* (2019) — thermoregulation chapter.
- Ramsey, M. et al. (2020) *Sci Rep* 10:9798 — vibrational swarm prediction.
- HiveEyes project (hiveeyes.org) — open hardware scale designs and
  telemetry discussions.
- Espressif ESP32 datasheet — sleep current figures.
- Avia Semiconductor HX711 datasheet — drift and noise specs.
