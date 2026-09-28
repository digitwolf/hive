//! I2S MEMS mic (INMP441) -> RMS level and per-band energy. Features only;
//! raw audio never leaves the node.

use esp_idf_hal::i2s::config::{
    ClockSource, Config, DataBitWidth, SlotMode, StdClkConfig, StdConfig, StdGpioConfig,
    StdSlotConfig,
};
use esp_idf_hal::i2s::{I2s, I2sDriver, I2sRx};
use esp_idf_hal::gpio::{InputPin, OutputPin};
use esp_idf_hal::peripheral::Peripheral;
use microfft::real::rfft_1024;

use crate::config::*;

pub struct SoundReading {
    pub rms_db: f32,
    pub bands_db: [f32; SOUND_BAND_COUNT],
    pub peak_hz: u32,
}

pub struct Sound<'d> {
    drv: I2sDriver<'d, I2sRx>,
}

fn to_db(x: f32) -> f32 {
    20.0 * (x + 1e-9).log10()
}

impl<'d> Sound<'d> {
    pub fn new<I: I2s>(
        i2s: impl Peripheral<P = I> + 'd,
        bclk: impl Peripheral<P = impl InputPin + OutputPin> + 'd,
        ws: impl Peripheral<P = impl InputPin + OutputPin> + 'd,
        din: impl Peripheral<P = impl InputPin> + 'd,
    ) -> anyhow::Result<Self> {
        let cfg = StdConfig::new(
            Config::default(),
            StdClkConfig::from_sample_rate_hz(SOUND_SAMPLE_RATE).clk_src(ClockSource::default()),
            StdSlotConfig::philips_slot_default(DataBitWidth::Bits32, SlotMode::Mono),
            StdGpioConfig::default(),
        );
        let mut drv = I2sDriver::new_std_rx(i2s, &cfg, bclk, din, None::<esp_idf_hal::gpio::AnyIOPin>, ws)?;
        drv.rx_enable()?;
        Ok(Sound { drv })
    }

    pub fn capture(&mut self) -> Option<SoundReading> {
        const N: usize = SOUND_FFT_N;
        let mut bytes = vec![0u8; N * 4];
        let mut frame = [0f32; N];

        // Discard ~100 ms while the mic settles.
        self.drv.read(&mut bytes, 200).ok()?;

        let frames = (SOUND_CAPTURE_MS * SOUND_SAMPLE_RATE / 1000) as usize / N;
        let mut band_acc = [0f32; SOUND_BAND_COUNT];
        let mut rms_acc = 0f64;
        let mut peak_mag = 0f32;
        let mut peak_bin = 0usize;
        let hz_per_bin = SOUND_SAMPLE_RATE as f32 / N as f32;

        for _ in 0..frames {
            let got = self.drv.read(&mut bytes, 500).ok()?;
            if got < bytes.len() {
                return None;
            }
            for (i, chunk) in bytes.chunks_exact(4).enumerate() {
                // 24-bit sample left-justified in 32 bits -> [-1, 1]
                let s = i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) >> 8;
                let v = s as f32 / 8_388_608.0;
                rms_acc += (v * v) as f64;
                // Hann window
                let w = 0.5 - 0.5 * (2.0 * core::f32::consts::PI * i as f32 / N as f32).cos();
                frame[i] = v * w;
            }
            let spectrum = rfft_1024(&mut frame);
            for b in 0..SOUND_BAND_COUNT {
                let lo = (SOUND_BANDS_HZ[b] as f32 / hz_per_bin) as usize;
                let hi = (SOUND_BANDS_HZ[b + 1] as f32 / hz_per_bin) as usize;
                for k in lo.max(1)..hi.min(spectrum.len()) {
                    let mag = spectrum[k].norm();
                    band_acc[b] += mag * mag;
                    if mag > peak_mag {
                        peak_mag = mag;
                        peak_bin = k;
                    }
                }
            }
        }

        let n = (frames * N) as f64;
        let mut bands_db = [0f32; SOUND_BAND_COUNT];
        for b in 0..SOUND_BAND_COUNT {
            bands_db[b] = ((to_db((band_acc[b] / frames as f32).sqrt()) * 10.0).round()) / 10.0;
        }
        Some(SoundReading {
            rms_db: to_db((rms_acc / n).sqrt() as f32),
            bands_db,
            peak_hz: (peak_bin as f32 * hz_per_bin) as u32,
        })
    }
}
