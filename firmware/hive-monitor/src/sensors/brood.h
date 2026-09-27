#pragma once

#include "../config.h"

struct BroodReading {
    float probes[BROOD_PROBE_COUNT];  // NaN for a missing probe
    float maxC;
    bool  ok;
};

void         broodInit();
BroodReading broodRead();
float        broodReadCompProbe();    // the HX711 temperature probe (last address)
