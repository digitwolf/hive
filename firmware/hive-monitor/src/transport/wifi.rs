//! Wi-Fi station + SNTP.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use esp_idf_hal::modem::Modem;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};

use crate::config::{CFG, WIFI_TIMEOUT_MS};

/// Anything before 2020-09-13 means the clock was never set.
const EPOCH_SANE: i64 = 1_600_000_000;

pub struct Net<'d> {
    wifi: BlockingWifi<EspWifi<'d>>,
}

impl<'d> Net<'d> {
    pub fn connect(
        modem: Modem,
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> anyhow::Result<Self> {
        let mut wifi = BlockingWifi::wrap(EspWifi::new(modem, sysloop.clone(), Some(nvs))?, sysloop)?;
        wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: CFG.wifi_ssid.try_into().map_err(|_| anyhow::anyhow!("ssid too long"))?,
            password: CFG.wifi_pass.try_into().map_err(|_| anyhow::anyhow!("password too long"))?,
            auth_method: if CFG.wifi_pass.is_empty() { AuthMethod::None } else { AuthMethod::WPA2Personal },
            ..Default::default()
        }))?;
        wifi.start()?;
        wifi.connect()?;
        // BlockingWifi::wait_netif_up has its own timeout; bound the whole thing anyway.
        let deadline = std::time::Instant::now() + Duration::from_millis(WIFI_TIMEOUT_MS as u64);
        loop {
            if wifi.is_connected()? && wifi.wifi().sta_netif().get_ip_info()?.ip != std::net::Ipv4Addr::UNSPECIFIED {
                break;
            }
            if std::time::Instant::now() > deadline {
                anyhow::bail!("wifi: timeout");
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(Net { wifi })
    }

    pub fn rssi(&self) -> i32 {
        // SAFETY: esp_wifi_sta_get_rssi writes an int on success.
        let mut rssi: i32 = 0;
        unsafe {
            esp_idf_sys::esp_wifi_sta_get_rssi(&mut rssi);
        }
        rssi
    }

    /// Block up to 5 s for an SNTP sync. Returns whether the clock is sane.
    pub fn ntp_sync(&self) -> bool {
        let Ok(sntp) = EspSntp::new_default() else { return false };
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while sntp.get_sync_status() != SyncStatus::Completed && std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(100));
        }
        epoch_now().is_some()
    }

    /// Heater builds stay up for weeks; re-associate if the AP dropped us.
    pub fn ensure_connected(&mut self) -> bool {
        if self.wifi.is_connected().unwrap_or(false) {
            return true;
        }
        log::warn!("wifi: reconnecting");
        self.wifi.connect().is_ok() && self.wifi.wait_netif_up().is_ok()
    }

    pub fn disconnect(mut self) {
        let _ = self.wifi.disconnect();
        let _ = self.wifi.stop();
    }
}

/// UTC epoch seconds if the clock has been set, else `None`.
pub fn epoch_now() -> Option<i64> {
    let s = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    (s >= EPOCH_SANE).then_some(s)
}
