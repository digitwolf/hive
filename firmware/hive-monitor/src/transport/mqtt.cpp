#include "mqtt.h"

#include <Arduino.h>
#include <ArduinoJson.h>
#include <PubSubClient.h>
#include <WiFi.h>
#include <math.h>
#include <time.h>

#include "../config.h"
#include "secrets.h"   // include/secrets.h, gitignored

static WiFiClient   net;
static PubSubClient mqtt(net);

static String topic(const char *leaf) {
    return String("hive/") + HIVE_ID + "/" + leaf;
}

bool wifiConnect() {
    if (WiFi.status() == WL_CONNECTED) return true;
    WiFi.mode(WIFI_STA);
    WiFi.begin(WIFI_SSID, WIFI_PASS);
    uint32_t t0 = millis();
    while (WiFi.status() != WL_CONNECTED && millis() - t0 < WIFI_TIMEOUT_MS) delay(100);
    return WiFi.status() == WL_CONNECTED;
}

bool ntpSync() {
    configTime(0, 0, "pool.ntp.org", "time.nist.gov");
    uint32_t t0 = millis();
    while (time(nullptr) < 1600000000 && millis() - t0 < 5000) delay(100);
    return time(nullptr) >= 1600000000;
}

bool mqttConnect() {
    if (mqtt.connected()) return true;
    mqtt.setServer(MQTT_HOST, MQTT_PORT);
    mqtt.setBufferSize(512);
    String will = topic("node");
    const char *user = strlen(MQTT_USER) ? MQTT_USER : nullptr;
    const char *pass = strlen(MQTT_PASS) ? MQTT_PASS : nullptr;
    uint32_t t0 = millis();
    while (!mqtt.connect(HIVE_ID, user, pass, will.c_str(), 1, false,
                         "{\"online\":false}")) {
        if (millis() - t0 > MQTT_TIMEOUT_MS) return false;
        delay(250);
    }
    return true;
}

void mqttDisconnect() {
    mqtt.loop();
    delay(50);   // let the last publish drain
    mqtt.disconnect();
}

// ---- helpers -------------------------------------------------------------

static void stamp(JsonDocument &d) {
    time_t now = time(nullptr);
    d["ts"] = (now >= 1600000000) ? (long)now : 0;
    d["fw"] = FW_VERSION;
}

static void putf(JsonDocument &d, const char *k, float v) {
    if (!isnan(v)) d[k] = roundf(v * 100.0f) / 100.0f;
}

static void send(const char *leaf, JsonDocument &d) {
    char buf[512];
    size_t n = serializeJson(d, buf, sizeof(buf));
    mqtt.publish(topic(leaf).c_str(), (const uint8_t *)buf, n, false);
    mqtt.loop();
}

// ---- publishers ----------------------------------------------------------

void mqttPublishEnv(const ClimateReading &r) {
    JsonDocument d;
    stamp(d);
    putf(d, "t_c", r.tC);
    putf(d, "rh", r.rh);
    putf(d, "t_out_c", r.tOutC);
    putf(d, "rh_out", r.rhOut);
    putf(d, "p_hpa", r.pHpa);
    send("env", d);
}

void mqttPublishWeight(const ScaleReading &r) {
    if (!r.ok) return;
    JsonDocument d;
    stamp(d);
    putf(d, "kg", r.kg);
    d["raw"] = r.raw;
    putf(d, "t_cell_c", r.tCellC);
    d["stable"] = r.stable;
    send("weight", d);
}

void mqttPublishBrood(const BroodReading &r) {
    if (!r.ok) return;
    JsonDocument d;
    stamp(d);
    JsonArray a = d["probes"].to<JsonArray>();
    for (int i = 0; i < BROOD_PROBE_COUNT; i++) {
        if (isnan(r.probes[i])) a.add(nullptr);
        else a.add(roundf(r.probes[i] * 100.0f) / 100.0f);
    }
    putf(d, "max_c", r.maxC);
    send("brood", d);
}

void mqttPublishSound(const SoundReading &r) {
    if (!r.ok) return;
    JsonDocument d;
    stamp(d);
    putf(d, "rms_db", r.rmsDb);
    JsonArray a = d["bands_db"].to<JsonArray>();
    for (int b = 0; b < SOUND_BAND_COUNT; b++) a.add(roundf(r.bandsDb[b] * 10.0f) / 10.0f);
    d["peak_hz"] = r.peakHz;
    send("sound", d);
}

void mqttPublishNode(const char *wakeReason) {
    JsonDocument d;
    stamp(d);
    d["online"]   = true;
    d["rssi"]     = WiFi.RSSI();
    d["uptime_s"] = millis() / 1000;
    d["heap"]     = ESP.getFreeHeap();
    d["reason"]   = wakeReason;
    // ADC through a 2:1 divider, 12-bit, ~3.3 V ref. Mains builds leave the
    // pin floating; server ignores vbat < 2.5.
    float vbat = analogRead(PIN_VBAT_ADC) / 4095.0f * 3.3f * 2.0f;
    putf(d, "vbat", vbat);
    send("node", d);
}

void mqttPublishEvent(const char *type) {
    JsonDocument d;
    stamp(d);
    d["type"] = type;
    send("event", d);
}

#ifdef HEATER_ENABLED
void mqttPublishHeater(const HeaterState &s) {
    JsonDocument d;
    stamp(d);
    d["enabled"]     = true;
    d["on"]          = s.on;
    putf(d, "t_pad_c", s.tPadC);
    d["setpoint_c"]  = HEATER_SETPOINT_C;
    putf(d, "duty_pct_1h", s.dutyPct1h);
    if (s.fault) d["fault"] = s.fault;
    send("heater", d);
}
#endif
