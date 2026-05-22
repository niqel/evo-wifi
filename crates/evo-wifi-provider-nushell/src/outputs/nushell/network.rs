use crate::values::{NushellRecord, NushellValue};
use evo_wifi_core::borrowed::WifiNetworkBorrowed;
use evo_wifi_core::contracts::WifiNetworkOutputContract;
use std::sync::OnceLock;

#[derive(Debug, Default)]
pub struct NushellNetworkOutputProvider {
    value: OnceLock<NushellValue>,
}

impl NushellNetworkOutputProvider {
    pub fn value(&self) -> Option<&NushellValue> {
        self.value.get()
    }

    pub fn into_value(self) -> Option<NushellValue> {
        self.value.into_inner()
    }
}

impl WifiNetworkOutputContract for NushellNetworkOutputProvider {
    fn provide(&self, networks: &[WifiNetworkBorrowed<'_>]) {
        let networks = networks
            .iter()
            .map(|network| {
                NushellRecord::from([
                    ("bssid", NushellValue::string(network.bssid)),
                    (
                        "frequency_mhz",
                        NushellValue::int(i64::from(network.frequency_mhz)),
                    ),
                    (
                        "signal_dbm",
                        NushellValue::int(i64::from(network.signal_dbm)),
                    ),
                    ("flags", NushellValue::string(network.flags)),
                    ("ssid", NushellValue::string(network.ssid)),
                ])
                .into()
            })
            .collect();

        let _ = self.value.set(NushellValue::list(networks));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_networks_as_nushell_list_of_records() {
        let provider = NushellNetworkOutputProvider::default();

        provider.provide(&[WifiNetworkBorrowed {
            bssid: "aa:bb:cc:dd:ee:ff",
            frequency_mhz: 5200,
            signal_dbm: -51,
            flags: "[WPA2-PSK-CCMP][ESS]",
            ssid: "evo-wifi",
        }]);

        assert_eq!(
            provider.into_value(),
            Some(NushellValue::list(vec![
                NushellRecord::from([
                    ("bssid", NushellValue::string("aa:bb:cc:dd:ee:ff")),
                    ("frequency_mhz", NushellValue::int(5200)),
                    ("signal_dbm", NushellValue::int(-51)),
                    ("flags", NushellValue::string("[WPA2-PSK-CCMP][ESS]")),
                    ("ssid", NushellValue::string("evo-wifi")),
                ])
                .into()
            ]))
        );
    }
}
