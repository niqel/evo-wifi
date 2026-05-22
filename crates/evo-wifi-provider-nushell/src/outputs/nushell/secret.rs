use crate::values::{NushellRecord, NushellValue};
use evo_wifi_core::borrowed::WifiSavedSecretBorrowed;
use evo_wifi_core::contracts::WifiSecretOutputContract;
use std::sync::OnceLock;

#[derive(Debug, Default)]
pub struct NushellSecretOutputProvider {
    value: OnceLock<NushellValue>,
}

impl NushellSecretOutputProvider {
    pub fn value(&self) -> Option<&NushellValue> {
        self.value.get()
    }

    pub fn into_value(self) -> Option<NushellValue> {
        self.value.into_inner()
    }
}

impl WifiSecretOutputContract for NushellSecretOutputProvider {
    fn provide(&self, secret: WifiSavedSecretBorrowed<'_>) {
        let value = NushellRecord::from([
            ("ssid", NushellValue::string(secret.ssid)),
            (
                "network_id",
                NushellValue::int(i64::from(secret.network_id)),
            ),
            ("secret", NushellValue::string(secret.secret)),
            ("secret_kind", NushellValue::string(secret.secret_kind)),
            ("source", NushellValue::string(secret.source)),
        ]);

        let _ = self.value.set(value.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_secret_as_nushell_record() {
        let provider = NushellSecretOutputProvider::default();

        provider.provide(WifiSavedSecretBorrowed {
            ssid: "evo-wifi",
            network_id: 7,
            secret: "secret",
            secret_kind: "password",
            source: "wpa_cli",
        });

        assert_eq!(
            provider.into_value(),
            Some(
                NushellRecord::from([
                    ("ssid", NushellValue::string("evo-wifi")),
                    ("network_id", NushellValue::int(7)),
                    ("secret", NushellValue::string("secret")),
                    ("secret_kind", NushellValue::string("password")),
                    ("source", NushellValue::string("wpa_cli")),
                ])
                .into()
            )
        );
    }
}
