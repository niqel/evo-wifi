use crate::borrowed::{WifiConnectionStatusBorrowed, WifiSavedNetworkBorrowed};

pub trait WifiForgetContract {
    fn provide(
        &self,
        network: WifiSavedNetworkBorrowed<'_>,
    ) -> Option<WifiConnectionStatusBorrowed<'_>>;
}
