// Bottom-board heater controller. Compiled only with -DHEATER_ENABLED.
// Design constraints: docs/research/hive-heating.md. This code is one of
// two limits; a hardware thermostat in series with the pad is the other.
#pragma once

struct HeaterState {
    bool        on;
    float       tPadC;       // NaN if probe missing
    float       dutyPct1h;   // rolling on-time over the last hour
    const char *fault;       // nullptr when healthy
};

void        heaterInit();
void        heaterTick();    // call every HEATER_TICK_MS
HeaterState heaterState();
