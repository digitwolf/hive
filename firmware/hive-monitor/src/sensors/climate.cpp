#include "climate.h"

#include <Adafruit_BME280.h>
#include <Adafruit_SHT31.h>
#include <Wire.h>
#include <math.h>

#include "../config.h"

static Adafruit_SHT31  sht;
static Adafruit_BME280 bme;
static bool haveSht = false, haveBme = false;

void climateInit() {
    Wire.begin(PIN_I2C_SDA, PIN_I2C_SCL);
    haveSht = sht.begin(0x44);
    haveBme = bme.begin(0x76) || bme.begin(0x77);
}

ClimateReading climateRead() {
    ClimateReading r = {NAN, NAN, NAN, NAN, NAN, false};
    if (haveSht) {
        r.tC = sht.readTemperature();
        r.rh = sht.readHumidity();
        r.ok = !isnan(r.tC);
    }
    if (haveBme) {
        r.tOutC = bme.readTemperature();
        r.rhOut = bme.readHumidity();
        r.pHpa  = bme.readPressure() / 100.0f;
    }
    return r;
}
