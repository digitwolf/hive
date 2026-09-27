#pragma once

struct ClimateReading {
    float tC, rh;             // under inner cover (SHT31)
    float tOutC, rhOut, pHpa; // outside (BME280); NaN if absent
    bool  ok;
};

void           climateInit();
ClimateReading climateRead();
