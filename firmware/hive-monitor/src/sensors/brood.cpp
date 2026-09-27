#include "brood.h"

#include <DallasTemperature.h>
#include <OneWire.h>
#include <math.h>

static OneWire           ow(PIN_ONEWIRE);
static DallasTemperature ds(&ow);
static const int         PROBE_TOTAL = sizeof(DS18B20_ADDRS) / sizeof(DS18B20_ADDRS[0]);

void broodInit() {
    ds.begin();
    ds.setResolution(12);
    ds.setWaitForConversion(true);
}

static float readAddr(int i) {
    if (i >= PROBE_TOTAL) return NAN;
    float t = ds.getTempC(DS18B20_ADDRS[i]);
    return (t == DEVICE_DISCONNECTED_C) ? NAN : t;
}

BroodReading broodRead() {
    BroodReading r;
    r.maxC = NAN;
    r.ok = false;
    ds.requestTemperatures();
    for (int i = 0; i < BROOD_PROBE_COUNT; i++) {
        r.probes[i] = readAddr(i);
        if (!isnan(r.probes[i])) {
            r.ok = true;
            if (isnan(r.maxC) || r.probes[i] > r.maxC) r.maxC = r.probes[i];
        }
    }
    return r;
}

float broodReadCompProbe() {
    ds.requestTemperatures();
    return readAddr(PROBE_TOTAL - 1);
}
