use crate::borrowed_data::WifiNetworkView;

pub trait WifiScanSystemWifiContract {
    fn provide(&self) -> Option<&[WifiNetworkView<'_>]>;
}
