#pragma once
#include <stdint.h>

struct ScaleReading {
    float   kg;        // tared, temperature-compensated
    long    raw;       // averaged HX711 counts (for server-side recalibration)
    float   tCellC;    // HX711 board temperature used for compensation (NaN if none)
    bool    stable;    // sample spread below SCALE_STABLE_KG
    bool    ok;
};

void         scaleInit();
long         scaleReadRaw();
ScaleReading scaleRead();
