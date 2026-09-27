#pragma once

#include "../config.h"

struct SoundReading {
    float rmsDb;
    float bandsDb[SOUND_BAND_COUNT];
    int   peakHz;
    bool  ok;
};

void         soundInit();
SoundReading soundCapture();
