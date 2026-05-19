use crate::borrowed_data::{WifiSavedNetworkView, WifiSavedSecretView};

pub trait WifiSavedSecretSystemWifiContract {
    fn resolve_saved_wifi_secret(
        &self,
        network: WifiSavedNetworkView<'_>,
    ) -> Option<WifiSavedSecretView<'_>>;
}
