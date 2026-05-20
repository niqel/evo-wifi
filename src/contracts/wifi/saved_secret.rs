use crate::borrowed::{WifiSavedNetworkBorrowed, WifiSavedSecretBorrowed};

pub trait WifiSavedSecretContract {
    fn provide(&self, network: WifiSavedNetworkBorrowed<'_>)
    -> Option<WifiSavedSecretBorrowed<'_>>;
}
