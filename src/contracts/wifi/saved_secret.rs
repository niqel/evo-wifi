use crate::borrowed_data::{WifiSavedNetworkView, WifiSavedSecretView};

pub trait WifiSavedSecretContract {
    fn provide(&self, network: WifiSavedNetworkView<'_>) -> Option<WifiSavedSecretView<'_>>;
}
