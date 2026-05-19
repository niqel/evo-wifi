use crate::borrowed_data::WifiNetworkView;

pub trait WifiNetworkRenderPresentationContract {
    fn provide(&self, networks: &[WifiNetworkView<'_>]);
}
