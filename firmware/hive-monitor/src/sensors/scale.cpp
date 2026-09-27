#include "scale.h"

#include <Arduino.h>
#include <HX711.h>
#include <limits.h>
#include <math.h>

#include "../config.h"
#include "brood.h"

static HX711 hx;

void scaleInit() {
    hx.begin(PIN_HX711_DOUT, PIN_HX711_SCK);
    hx.power_up();
}

long scaleReadRaw() {
    if (!hx.wait_ready_timeout(1000)) return 0;
    return hx.read_average(SCALE_SAMPLES);
}

ScaleReading scaleRead() {
    ScaleReading r = {NAN, 0, NAN, false, false};
    if (!hx.wait_ready_timeout(1000)) return r;

    long minRaw = LONG_MAX, maxRaw = LONG_MIN;
    int64_t sum = 0;
    for (int i = 0; i < SCALE_SAMPLES; i++) {
        long v = hx.read();
        sum += v;
        if (v < minRaw) minRaw = v;
        if (v > maxRaw) maxRaw = v;
    }
    r.raw = (long)(sum / SCALE_SAMPLES);

    float kg = (r.raw - SCALE_OFFSET) / SCALE_SCALE;
    r.tCellC = broodReadCompProbe();
    if (!isnan(r.tCellC)) kg -= SCALE_TEMP_COEF * (r.tCellC - SCALE_TEMP_REF_C);

    r.kg = kg;
    r.stable = ((maxRaw - minRaw) / SCALE_SCALE) < SCALE_STABLE_KG;
    r.ok = true;
    hx.power_down();   // saves ~1.5 mA during deep sleep
    return r;
}
