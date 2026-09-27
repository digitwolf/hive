#ifdef HEATER_ENABLED
#include "heater.h"

#include <Arduino.h>
#include <esp_task_wdt.h>
#include <math.h>

#include "../config.h"
#include "../sensors/brood.h"

static HeaterState st = {false, NAN, 0.0f, nullptr};
static uint32_t    lastGoodProbeMs = 0;

// 1-hour rolling duty: one slot per tick.
static const int SLOTS = 3600 * 1000 / HEATER_TICK_MS;
static uint8_t   onHist[SLOTS / 8 + 1];
static int       slot = 0;
static int       onCount = 0;

static void drive(bool on) {
    digitalWrite(PIN_HEATER, on ? HIGH : LOW);
    st.on = on;
}

static void recordDuty(bool on) {
    int  byte = slot / 8, bit = slot % 8;
    bool was  = onHist[byte] & (1 << bit);
    if (was) onCount--;
    if (on)  { onHist[byte] |= (1 << bit); onCount++; }
    else       onHist[byte] &= ~(1 << bit);
    slot = (slot + 1) % SLOTS;
    st.dutyPct1h = 100.0f * onCount / SLOTS;
}

void heaterInit() {
    pinMode(PIN_HEATER, OUTPUT);
    drive(false);
    // Hardware watchdog: if the tick stops, the chip resets and the pin
    // comes up LOW (heater off).
    esp_task_wdt_init(10, true);
    esp_task_wdt_add(nullptr);
}

void heaterTick() {
    esp_task_wdt_reset();

    // Pad temperature: the last DS18B20 in the list is repurposed on heater
    // builds (see hardware/BOM.md). Reading uses the comp-probe helper.
    float t = broodReadCompProbe();
    if (!isnan(t)) {
        st.tPadC = t;
        lastGoodProbeMs = millis();
    }

    // Fail-off conditions, checked before any "on" decision.
    if (isnan(st.tPadC) || millis() - lastGoodProbeMs > HEATER_PROBE_STALE_MS) {
        st.fault = "probe";
        drive(false);
    } else if (st.tPadC >= HEATER_MAX_PAD_C) {
        st.fault = "overtemp";
        drive(false);
    } else {
        st.fault = nullptr;
        if (!st.on && st.tPadC < HEATER_SETPOINT_C - HEATER_HYSTERESIS_C) drive(true);
        if ( st.on && st.tPadC > HEATER_SETPOINT_C + HEATER_HYSTERESIS_C) drive(false);
    }
    recordDuty(st.on);
}

HeaterState heaterState() { return st; }
#endif
