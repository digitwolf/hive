# hive-monitor (ESP32, Rust)

Status: **v0 skeleton, not yet compiled or run on hardware.** The structure
and the MQTT contract are set; sensor drivers need bench validation.

## Why Rust / `esp-idf-svc`

Two Rust routes exist for the ESP32:

| | `esp-idf-svc` (std, on ESP-IDF) | `esp-hal` + embassy (bare-metal, no_std) |
|---|---|---|
| Wi-Fi, MQTT, SNTP | Mature, from ESP-IDF | `esp-wifi` works but MQTT/SNTP are DIY |
| I2S, deep sleep, ext0 wake | Wrapped | Available, lower level |
| Binary size / boot time | Bigger, slower | Small, fast |
| Ecosystem risk | Low | Moving fast |

This node needs Wi-Fi + MQTT + I2S + deep sleep and spends 99 % of its life
asleep, so boot time and binary size don't matter. `esp-idf-svc` it is.
Revisit if a battery build makes wake-time energy matter.

## Toolchain

The ESP32 is Xtensa, which needs Espressif's Rust fork:

```sh
cargo install espup ldproxy espflash
espup install                 # installs the `esp` toolchain
. ~/export-esp.sh             # every shell
```

Requires Python 3 and git (ESP-IDF v5.2 is fetched by `embuild` on first
build into `.embuild/`). First build takes a while.

## Build / flash

```sh
cp cfg.toml.example cfg.toml     # hive_id + Wi-Fi + MQTT
cargo build --release
cargo run --release              # = espflash flash --monitor
cargo build --release --features heater   # heater controller build (hive-a only)
```

One `cfg.toml` per hive: edit `hive_id` (and nothing else) between builds.

## Loop

```
boot → read all sensors (~3 s) → Wi-Fi → NTP (first boot / every 24 h)
     → publish env/weight/brood/sound/node → MQTT disconnect
     → deep sleep SAMPLE_INTERVAL_S (default 300)
```

The lid reed switch is an EXT0 wake source so an inspection publishes an
`event` immediately and marks weight readings unstable while open.

With `--features heater` the node does **not** deep-sleep (the controller
runs continuously): 1 s control ticks with the task watchdog armed,
publishing on the same schedule. The heater fails *off* on a missing/stale
probe, on over-temperature, and on a watchdog reset. A hardware thermostat
in series is still required — see `docs/research/hive-heating.md`.

## Layout

```
src/
  main.rs            boot, sample, publish, sleep (orchestration only)
  config.rs          pins, intervals, calibration, DS18B20 addresses, cfg.toml
  telemetry.rs       serde payload structs = the wire schema
  sensors/
    scale.rs         HX711 bit-bang + temperature compensation
    climate.rs       SHT31 (inside) + BME280 (outside) on one I2C bus
    brood.rs         DS18B20 1-Wire bus (+ bus scan helper)
    sound.rs         I2S mic → RMS + band energies (microfft)
    lid.rs           reed switch + EXT0 wake
  transport/
    wifi.rs          Wi-Fi STA + SNTP
    mqtt.rs          MQTT publish, one fn per topic
  heater.rs          bang-bang controller with limits (feature "heater")
```

## First-time setup on the bench

1. **DS18B20 addresses:** on a cold boot the node logs every 1-Wire ROM
   address it finds. Paste them into `DS18B20_ADDRS` in `config.rs` in
   the order: brood probes first, HX711 comp probe last.
2. **Scale calibration:** set `SCALE_CAL_MODE = true`, build, note `raw`
   with the empty platform (`raw_0`) and with a known mass (`raw_20` for
   20 kg of sugar). `SCALE_SCALE = (raw_20 - raw_0) / 20.0`,
   `SCALE_OFFSET = raw_0`. Set `SCALE_CAL_MODE` back to `false`.
3. **Temperature compensation:** leave the loaded platform through a day
   with a big temperature swing, fit `kg` vs `t_cell_c` in Grafana, set
   `SCALE_TEMP_COEF`.

## Known gaps (v0)

- Not compiled yet: crate API details (`esp-idf-hal` I2S config builder,
  `bme280`/`one-wire-bus` trait versions) are from memory and will need
  touching up on the first `cargo build`.
- No OTA. Flash over USB for now.
