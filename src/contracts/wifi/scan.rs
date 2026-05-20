use crate::borrowed_data::WifiNetworkView;

pub trait WifiScanContract {
    fn provide(&self) -> Option<&[WifiNetworkView<'_>]>;
}
