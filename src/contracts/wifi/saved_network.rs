use crate::borrowed::{
    WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed, WifiSavedNetworkBorrowed,
};

pub trait WifiSavedNetworkContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        selection: WifiNetworkSelectionInputBorrowed<'_>,
        next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
    ) -> Option<R>;
}
