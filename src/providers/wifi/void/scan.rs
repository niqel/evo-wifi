use crate::borrowed::WifiNetworkBorrowed;
use crate::contracts::WifiScanContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiScanProvider;

impl WifiScanContract for VoidWifiScanProvider {
    fn provide(&self) -> Option<&[WifiNetworkBorrowed<'_>]> {
        None
    }
}
