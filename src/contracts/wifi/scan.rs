use crate::borrowed::WifiNetworkBorrowed;

pub trait WifiScanContract {
    fn provide(&self) -> Option<&[WifiNetworkBorrowed<'_>]>;
}
