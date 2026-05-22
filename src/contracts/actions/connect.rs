use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiSavedNetworkBorrowed,
};

pub trait WifiConnectContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        network: WifiSavedNetworkBorrowed<'_>,
        next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R>;
}
