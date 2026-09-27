#include "lid.h"

#include <Arduino.h>
#include <esp_sleep.h>

#include "../config.h"

void lidInit() { pinMode(PIN_LID, INPUT_PULLUP); }

// Reed switch closed (magnet present, lid on) pulls the pin LOW.
bool lidIsOpen() { return digitalRead(PIN_LID) == HIGH; }

void lidArmWake() {
    // Wake on the opposite level from the current one, so either edge wakes us.
    esp_sleep_enable_ext0_wakeup((gpio_num_t)PIN_LID, lidIsOpen() ? 0 : 1);
}
