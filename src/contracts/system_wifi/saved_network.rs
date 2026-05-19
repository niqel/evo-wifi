use crate::borrowed_data::{WifiNetworkSelectionInputView, WifiSavedNetworkView};

pub trait WifiSavedNetworkSystemWifiContract {
    fn resolve_saved_wifi_network(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
    ) -> Option<WifiSavedNetworkView<'_>>;
}
