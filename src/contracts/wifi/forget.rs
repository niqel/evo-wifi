use crate::borrowed_data::{WifiConnectionStatusView, WifiSavedNetworkView};

pub trait WifiForgetContract {
    fn provide(&self, network: WifiSavedNetworkView<'_>) -> Option<WifiConnectionStatusView<'_>>;
}
