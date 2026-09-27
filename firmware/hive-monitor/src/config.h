// Pins, timing, and calibration for the hive-monitor node.
// Per-hive values (HIVE_ID, calibration) are the ones you'll actually edit.
#pragma once

#ifndef HIVE_ID
#define HIVE_ID "hive-x"
#endif
#ifndef FW_VERSION
#define FW_VERSION "dev"
#endif

// ---- timing --------------------------------------------------------------
#define SAMPLE_INTERVAL_S      300      // deep-sleep period between samples
#define NTP_RESYNC_S           86400
#define WIFI_TIMEOUT_MS        15000
#define MQTT_TIMEOUT_MS        5000

// ---- pins (ESP32 devkit) -------------------------------------------------
#define PIN_HX711_DOUT         16
#define PIN_HX711_SCK          4
#define PIN_ONEWIRE            15
#define PIN_I2C_SDA            21
#define PIN_I2C_SCL            22
#define PIN_I2S_WS             25
#define PIN_I2S_SCK            26
#define PIN_I2S_SD             33
#define PIN_LID                27       // reed switch to GND, internal pull-up; RTC-capable for EXT0 wake
#define PIN_VBAT_ADC           34       // through 2:1 divider; leave unconnected on mains builds
#define PIN_HEATER             32       // MOSFET gate (HEATER_ENABLED only)

// ---- scale ---------------------------------------------------------------
#define SCALE_CAL_MODE         0        // 1 = print raw only, no publish
#define SCALE_OFFSET           0L       // raw reading with empty platform
#define SCALE_SCALE            1.0f     // raw counts per kg
#define SCALE_TEMP_COEF        0.0f     // kg per °C, fitted from a temp-swing day
#define SCALE_TEMP_REF_C       20.0f
#define SCALE_SAMPLES          16
#define SCALE_STABLE_KG        0.05f    // max spread across samples to call it "stable"

// ---- DS18B20 -------------------------------------------------------------
// Order matters: index 0.. are brood probes (published), last one is the
// HX711 compensation probe. Fill in from a one-time bus scan.
#define BROOD_PROBE_COUNT      2
static const uint8_t DS18B20_ADDRS[][8] = {
    {0x28, 0, 0, 0, 0, 0, 0, 0},        // brood probe 0 (top of nest)
    {0x28, 0, 0, 0, 0, 0, 0, 0},        // brood probe 1
    {0x28, 0, 0, 0, 0, 0, 0, 0},        // HX711 board temperature
};

// ---- sound ---------------------------------------------------------------
#define SOUND_SAMPLE_RATE      8000
#define SOUND_FFT_N            1024     // ~8 Hz bins
#define SOUND_CAPTURE_MS       2000
// Band edges in Hz; must match telemetry-schema.md
static const int SOUND_BANDS_HZ[] = {0, 100, 200, 300, 500, 1000, 2000};
#define SOUND_BAND_COUNT       6

// ---- heater (HEATER_ENABLED only) ----------------------------------------
// The firmware setpoint is NOT the only limit: a hardware thermostat in
// series must open above HEATER_HW_CUTOFF_C. See docs/research/hive-heating.md.
#define HEATER_SETPOINT_C      5.0f
#define HEATER_HYSTERESIS_C    1.0f
#define HEATER_MAX_PAD_C       10.0f    // firmware hard-off
#define HEATER_HW_CUTOFF_C     15.0f    // documented hardware thermostat rating
#define HEATER_TICK_MS         1000
#define HEATER_PROBE_STALE_MS  30000    // no valid pad temp for this long -> off
