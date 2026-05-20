use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiNetworkSelectionInputBorrowed, WifiPasswordInputBorrowed,
};

pub trait WifiConnectContract {
    fn provide(
        &self,
        selection: WifiNetworkSelectionInputBorrowed<'_>,
        password: Option<WifiPasswordInputBorrowed<'_>>,
    ) -> Option<WifiConnectionStatusBorrowed<'_>>;
}
