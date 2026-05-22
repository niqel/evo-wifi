use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed,
    WifiPasswordInputBorrowed,
};

pub trait WifiNewNetworkConnectContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        selection: WifiNetworkSelectionInputBorrowed<'_>,
        password: WifiPasswordInputBorrowed<'_>,
        next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R>;
}
