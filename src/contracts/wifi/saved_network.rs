use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiSavedNetworkBorrowed,
};

pub trait WifiSavedNetworkContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        status: WifiConnectionStatusBorrowed<'_>,
        next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
    ) -> Option<R>;
}
