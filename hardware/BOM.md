# Bill of materials — monitor node v0 (per hive)

Prices are rough 2026 hobby-supplier figures; update when ordered.
Target: under ~$120/hive so the DIY beats a BroodMinder-class scale.

| # | Item | Qty | ~USD | Notes |
|---|------|-----|------|-------|
| 1 | ESP32 dev board (ESP32-WROOM-32, or FireBeetle if battery) | 1 | 8 | Battery build needs a low-quiescent board |
| 2 | 50 kg half-bridge load cell | 4 | 8 | Buy 5 (one spare / test) |
| 3 | HX711 amplifier board | 1 | 3 | Green "sparkfun-style" boards are fine; buy 2 |
| 4 | SHT31 (or SHT40) temp/RH breakout with PTFE cap | 1 | 10 | Under inner cover |
| 5 | BME280 breakout | 1 | 6 | Outside, in a radiation shield (stacked plates) |
| 6 | DS18B20 waterproof probe, 1 m | 3 | 9 | 1 brood nest, 1 HX711 comp, 1 spare/heater pad |
| 7 | INMP441 I2S microphone | 1 | 4 | |
| 8 | Reed switch + magnet | 1 | 2 | Lid open |
| 9 | 4.7 kΩ resistor (1-Wire pull-up), misc resistors | — | 1 | |
| 10 | IP66 enclosure 150×100×70 | 1 | 10 | |
| 11 | Small IP65 box for HX711 | 1 | 4 | |
| 12 | PG7 cable glands | 5 | 3 | |
| 13 | Vent plug (PTFE membrane) | 1 | 3 | |
| 14 | Shielded 4-core cable, 3 m | 1 | 5 | Load cells → HX711 → node |
| 15 | 3/4" exterior plywood 600×500, 2 pcs | 1 | 15 | Platform |
| 16 | Corner mounts for load cells (3D-printed or wood) | 4 | 2 | |
| 17 | 12 V 2 A outdoor PSU **or** 2× 18650 + BMS + 6 V panel + charger | 1 | 15 / 35 | Decide after site check |
| 18 | 12→5 V buck converter | 1 | 3 | |
| 19 | Fuse holder + 2 A fuse | 1 | 2 | |
| | **Total (mains)** | | **~110** | |

## Heater branch (only if/when built, per `docs/research/hive-heating.md`)

| Item | Qty | ~USD |
|------|-----|------|
| 12 V silicone heating pad, 10–20 W, ~100×150 mm | 1 | 10 |
| KSD9700-type NC thermostat, 15 °C (or PTC self-regulating pad instead) | 1 | 2 |
| Logic-level MOSFET module / SSR | 1 | 3 |
| 3 A fuse + holder | 1 | 2 |
| Toggle switch (manual off) | 1 | 2 |
| Aluminium plate to spread heat over bottom board | 1 | 5 |
