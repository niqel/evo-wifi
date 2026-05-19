use crate::borrowed_data::{WifiSavedNetworkView, WifiSavedSecretView};

pub trait WifiSavedSecretSystemWifiContract {
    fn provide(&self, network: WifiSavedNetworkView<'_>) -> Option<WifiSavedSecretView<'_>>;
}
