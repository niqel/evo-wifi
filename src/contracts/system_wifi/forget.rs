use crate::borrowed_data::{WifiConnectionStatusView, WifiSavedNetworkView};

pub trait WifiForgetSystemWifiContract {
    fn forget_wifi_network(
        &self,
        network: WifiSavedNetworkView<'_>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}
