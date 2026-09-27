# hive-monitor (ESP32)

Status: **v0 skeleton, not yet compiled or run on hardware.** The structure
and the MQTT contract are set; sensor drivers are minimal and need bench
validation.

## Build

```sh
cp include/secrets.h.example include/secrets.h   # Wi-Fi + MQTT
pio run -e hive-a            # or hive-b
pio run -e hive-a -t upload
pio device monitor
```

Environments differ only in `HIVE_ID`. `-e hive-a-heater` adds
`HEATER_ENABLED=1`.

## Loop

```
boot → read all sensors (~3 s) → Wi-Fi → NTP (first boot / every 24 h)
     → publish env/weight/brood/sound/node → MQTT disconnect
     → deep sleep SAMPLE_INTERVAL_S (default 300)
```

Lid reed switch is an EXT0 wake source so an inspection publishes an
`event` immediately and suppresses "stable" weight readings while open.

With `HEATER_ENABLED` the node does **not** deep-sleep (the controller needs
to run continuously); it light-sleeps between 1 s control ticks and
publishes on the same schedule.

## Layout

```
src/
  main.cpp            boot, sample, publish, sleep
  config.h            pins, intervals, calibration constants, DS18B20 addresses
  sensors/
    scale.{h,cpp}     HX711 + temperature compensation + tare
    climate.{h,cpp}   SHT31 (inside) + BME280 (outside)
    brood.{h,cpp}     DS18B20 bus
    sound.{h,cpp}     I2S mic → RMS + band energies
    lid.{h,cpp}       reed switch
  transport/
    mqtt.{h,cpp}      Wi-Fi + MQTT + JSON helpers
  heater/
    heater.{h,cpp}    bang-bang controller with limits (behind HEATER_ENABLED)
include/
  secrets.h.example
```

## Calibration

1. With the empty platform, run once with `SCALE_CAL_MODE=1`; note `raw`.
2. Put a known mass (e.g. 20 kg of sugar) on; note `raw`.
3. `SCALE_SCALE = (raw_20 - raw_0) / 20.0`, `SCALE_OFFSET = raw_0` in
   `config.h`.
4. Temperature compensation: leave the loaded platform through a day with
   a big temperature swing, fit `kg` vs `t_cell_c`, set `SCALE_TEMP_COEF`.
