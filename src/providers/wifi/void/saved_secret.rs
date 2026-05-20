use crate::borrowed::{WifiSavedNetworkBorrowed, WifiSavedSecretBorrowed};
use crate::contracts::WifiSavedSecretContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiSavedSecretProvider;

impl WifiSavedSecretContract for VoidWifiSavedSecretProvider {
    fn provide(
        &self,
        _network: WifiSavedNetworkBorrowed<'_>,
    ) -> Option<WifiSavedSecretBorrowed<'_>> {
        None
    }
}
