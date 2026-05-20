use crate::borrowed_data::{WifiNetworkSelectionInputView, WifiSavedNetworkView};

pub trait WifiSavedNetworkContract {
    fn provide(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
    ) -> Option<WifiSavedNetworkView<'_>>;
}
