// hive-monitor: boot -> sample -> publish -> deep sleep.
// Keep this file to orchestration; details live in sensors/ and transport/.
#include <Arduino.h>
#include <esp_sleep.h>

#include "config.h"
#include "sensors/scale.h"
#include "sensors/climate.h"
#include "sensors/brood.h"
#include "sensors/sound.h"
#include "sensors/lid.h"
#include "transport/mqtt.h"
#ifdef HEATER_ENABLED
#include "heater/heater.h"
#endif

RTC_DATA_ATTR static uint32_t bootCount = 0;
RTC_DATA_ATTR static time_t lastNtpSync = 0;

static const char *wakeReason() {
    switch (esp_sleep_get_wakeup_cause()) {
        case ESP_SLEEP_WAKEUP_EXT0:  return "lid";
        case ESP_SLEEP_WAKEUP_TIMER: return "timer";
        default:                     return "boot";
    }
}

static void sampleAndPublish(bool lidOpen) {
    ScaleReading   scale = scaleRead();
    ClimateReading env   = climateRead();
    BroodReading   brood = broodRead();
    SoundReading   snd   = soundCapture();

    if (lidOpen) scale.stable = false;   // inspection in progress

    if (!mqttConnect()) return;
    mqttPublishEnv(env);
    mqttPublishWeight(scale);
    mqttPublishBrood(brood);
    mqttPublishSound(snd);
#ifdef HEATER_ENABLED
    mqttPublishHeater(heaterState());
#endif
    mqttPublishNode(wakeReason());
    mqttDisconnect();
}

void setup() {
    Serial.begin(115200);
    bootCount++;

    lidInit();
    scaleInit();
    climateInit();
    broodInit();
    soundInit();
#ifdef HEATER_ENABLED
    heaterInit();
#endif

    const bool lidOpen = lidIsOpen();
    const char *reason = wakeReason();

    if (!wifiConnect()) {
        // No network: still run the heater loop if enabled, else sleep and retry.
        Serial.println("wifi: failed");
    } else {
        if (lastNtpSync == 0 || time(nullptr) - lastNtpSync > NTP_RESYNC_S) {
            if (ntpSync()) lastNtpSync = time(nullptr);
        }
        if (strcmp(reason, "boot") == 0 && mqttConnect()) {
            mqttPublishEvent("boot");
            mqttDisconnect();
        }
        if (strcmp(reason, "lid") == 0 && mqttConnect()) {
            mqttPublishEvent(lidOpen ? "lid_open" : "lid_closed");
            mqttDisconnect();
        }
    }

#if SCALE_CAL_MODE
    while (true) {
        Serial.printf("raw=%ld\n", scaleReadRaw());
        delay(1000);
    }
#endif

    sampleAndPublish(lidOpen);

#ifndef HEATER_ENABLED
    lidArmWake();
    esp_sleep_enable_timer_wakeup((uint64_t)SAMPLE_INTERVAL_S * 1000000ULL);
    Serial.printf("sleep %d s (boot #%u)\n", SAMPLE_INTERVAL_S, bootCount);
    Serial.flush();
    esp_deep_sleep_start();
#endif
}

void loop() {
#ifdef HEATER_ENABLED
    // Heater builds stay awake: control tick every second, publish on schedule.
    static uint32_t lastSample = 0;
    heaterTick();
    if (millis() - lastSample > (uint32_t)SAMPLE_INTERVAL_S * 1000UL) {
        lastSample = millis();
        if (wifiConnect()) sampleAndPublish(lidIsOpen());
    }
    delay(HEATER_TICK_MS);
#endif
}
