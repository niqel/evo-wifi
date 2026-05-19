use crate::borrowed_data::WifiNetworkView;

pub trait WifiNetworkRenderPresentationContract {
    fn render_wifi_networks(&self, networks: &[WifiNetworkView<'_>]);
}
