use crate::borrowed_data::WifiNetworkView;

pub trait WifiScanSystemWifiContract {
    fn scan_wifi_networks(&self) -> Option<&[WifiNetworkView<'_>]>;
}
