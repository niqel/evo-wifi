use crate::borrowed_data::WifiNetworkView;

pub trait WifiNetworkOutputContract {
    fn provide(&self, networks: &[WifiNetworkView<'_>]);
}
