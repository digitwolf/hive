// I2S MEMS mic -> RMS level and per-band energy. Features only; raw audio
// never leaves the node.
#include "sound.h"

#include <Arduino.h>
#include <arduinoFFT.h>
#include <driver/i2s.h>
#include <math.h>

static float vReal[SOUND_FFT_N];
static float vImag[SOUND_FFT_N];
static ArduinoFFT<float> fft(vReal, vImag, SOUND_FFT_N, SOUND_SAMPLE_RATE);

void soundInit() {
    i2s_config_t cfg = {};
    cfg.mode                 = (i2s_mode_t)(I2S_MODE_MASTER | I2S_MODE_RX);
    cfg.sample_rate          = SOUND_SAMPLE_RATE;
    cfg.bits_per_sample      = I2S_BITS_PER_SAMPLE_32BIT;
    cfg.channel_format       = I2S_CHANNEL_FMT_ONLY_LEFT;
    cfg.communication_format = I2S_COMM_FORMAT_STAND_I2S;
    cfg.dma_buf_count        = 4;
    cfg.dma_buf_len          = 256;
    i2s_driver_install(I2S_NUM_0, &cfg, 0, nullptr);

    i2s_pin_config_t pins = {};
    pins.bck_io_num   = PIN_I2S_SCK;
    pins.ws_io_num    = PIN_I2S_WS;
    pins.data_out_num = I2S_PIN_NO_CHANGE;
    pins.data_in_num  = PIN_I2S_SD;
    i2s_set_pin(I2S_NUM_0, &pins);
}

static float toDb(float x) { return 20.0f * log10f(x + 1e-9f); }

SoundReading soundCapture() {
    SoundReading r = {};
    static int32_t buf[SOUND_FFT_N];
    size_t got = 0;

    // Discard the first ~100 ms while the mic settles.
    i2s_read(I2S_NUM_0, buf, sizeof(buf), &got, pdMS_TO_TICKS(200));

    const int frames = (SOUND_CAPTURE_MS * SOUND_SAMPLE_RATE / 1000) / SOUND_FFT_N;
    float bandAcc[SOUND_BAND_COUNT] = {0};
    double rmsAcc = 0;
    float  peakMag = 0;
    int    peakBin = 0;

    for (int f = 0; f < frames; f++) {
        if (i2s_read(I2S_NUM_0, buf, sizeof(buf), &got, pdMS_TO_TICKS(500)) != ESP_OK) return r;
        for (int i = 0; i < SOUND_FFT_N; i++) {
            vReal[i] = (float)(buf[i] >> 8) / 8388608.0f;   // 24-bit sample -> [-1, 1]
            vImag[i] = 0;
            rmsAcc += (double)vReal[i] * vReal[i];
        }
        fft.windowing(FFTWindow::Hann, FFTDirection::Forward);
        fft.compute(FFTDirection::Forward);
        fft.complexToMagnitude();

        const float hzPerBin = (float)SOUND_SAMPLE_RATE / SOUND_FFT_N;
        for (int b = 0; b < SOUND_BAND_COUNT; b++) {
            int lo = (int)(SOUND_BANDS_HZ[b] / hzPerBin);
            int hi = (int)(SOUND_BANDS_HZ[b + 1] / hzPerBin);
            for (int k = max(lo, 1); k < hi && k < SOUND_FFT_N / 2; k++) {
                bandAcc[b] += vReal[k] * vReal[k];
                if (vReal[k] > peakMag) { peakMag = vReal[k]; peakBin = k; }
            }
        }
    }

    const int n = frames * SOUND_FFT_N;
    r.rmsDb = toDb(sqrtf((float)(rmsAcc / n)));
    for (int b = 0; b < SOUND_BAND_COUNT; b++) r.bandsDb[b] = toDb(sqrtf(bandAcc[b] / frames));
    r.peakHz = (int)(peakBin * (float)SOUND_SAMPLE_RATE / SOUND_FFT_N);
    r.ok = true;
    return r;
}
