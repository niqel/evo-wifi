use crate::borrowed::{WifiInterfaceBorrowed, WifiSavedNetworkBorrowed, WifiSavedSecretBorrowed};

pub trait WifiSavedSecretContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        network: WifiSavedNetworkBorrowed<'_>,
        next: impl FnOnce(WifiSavedSecretBorrowed<'_>) -> R,
    ) -> Option<R>;
}
