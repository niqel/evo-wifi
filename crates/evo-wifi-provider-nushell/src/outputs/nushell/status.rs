use crate::values::{NushellRecord, NushellValue};
use evo_wifi_core::borrowed::WifiConnectionStatusBorrowed;
use evo_wifi_core::contracts::WifiStatusOutputContract;
use std::sync::OnceLock;

#[derive(Debug, Default)]
pub struct NushellStatusOutputProvider {
    value: OnceLock<NushellValue>,
}

impl NushellStatusOutputProvider {
    pub fn value(&self) -> Option<&NushellValue> {
        self.value.get()
    }

    pub fn into_value(self) -> Option<NushellValue> {
        self.value.into_inner()
    }
}

impl WifiStatusOutputContract for NushellStatusOutputProvider {
    fn provide(&self, status: WifiConnectionStatusBorrowed<'_>) {
        let value = NushellRecord::from([
            ("ssid", NushellValue::string(status.ssid)),
            ("state", NushellValue::string(status.state.as_wpa_state())),
        ]);

        let _ = self.value.set(value.into());
    }
}

impl WifiStatusOutputContract for &NushellStatusOutputProvider {
    fn provide(&self, status: WifiConnectionStatusBorrowed<'_>) {
        (*self).provide(status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evo_wifi_core::borrowed::WifiConnectionState;

    #[test]
    fn provides_status_as_nushell_record() {
        let provider = NushellStatusOutputProvider::default();

        provider.provide(WifiConnectionStatusBorrowed {
            ssid: "evo-wifi",
            state: WifiConnectionState::Completed,
        });

        assert_eq!(
            provider.into_value(),
            Some(
                NushellRecord::from([
                    ("ssid", NushellValue::string("evo-wifi")),
                    ("state", NushellValue::string("COMPLETED")),
                ])
                .into()
            )
        );
    }
}
