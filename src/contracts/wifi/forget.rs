use crate::borrowed::{WifiInterfaceBorrowed, WifiSavedNetworkBorrowed};

pub trait WifiForgetContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        network: WifiSavedNetworkBorrowed<'_>,
        next: impl FnOnce() -> R,
    ) -> Option<R>;
}
