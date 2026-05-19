use crate::borrowed_data::{WifiConnectionStatusView, WifiSavedNetworkView};

pub trait WifiForgetSystemWifiContract {
    fn provide(&self, network: WifiSavedNetworkView<'_>) -> Option<WifiConnectionStatusView<'_>>;
}
