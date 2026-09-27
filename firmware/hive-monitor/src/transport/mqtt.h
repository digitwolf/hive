#pragma once

#include "../sensors/scale.h"
#include "../sensors/climate.h"
#include "../sensors/brood.h"
#include "../sensors/sound.h"
#ifdef HEATER_ENABLED
#include "../heater/heater.h"
#endif

bool wifiConnect();
bool ntpSync();
bool mqttConnect();
void mqttDisconnect();

// One function per topic in docs/research/telemetry-schema.md.
void mqttPublishEnv(const ClimateReading &r);
void mqttPublishWeight(const ScaleReading &r);
void mqttPublishBrood(const BroodReading &r);
void mqttPublishSound(const SoundReading &r);
void mqttPublishNode(const char *wakeReason);
void mqttPublishEvent(const char *type);
#ifdef HEATER_ENABLED
void mqttPublishHeater(const HeaterState &s);
#endif
