use crate::borrowed::{WifiNetworkSelectionInputBorrowed, WifiSavedNetworkBorrowed};

pub trait WifiSavedNetworkContract {
    fn provide(
        &self,
        selection: WifiNetworkSelectionInputBorrowed<'_>,
    ) -> Option<WifiSavedNetworkBorrowed<'_>>;
}
