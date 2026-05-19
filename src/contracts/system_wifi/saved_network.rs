use crate::borrowed_data::{WifiNetworkSelectionInputView, WifiSavedNetworkView};

pub trait WifiSavedNetworkSystemWifiContract {
    fn provide(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
    ) -> Option<WifiSavedNetworkView<'_>>;
}
