use crate::borrowed::{WifiInterfaceBorrowed, WifiNetworkBorrowed};

pub trait WifiScanContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        next: impl FnOnce(&[WifiNetworkBorrowed<'_>]) -> R,
    ) -> Option<R>;
}
