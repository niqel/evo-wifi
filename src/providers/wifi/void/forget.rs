use crate::borrowed::{WifiConnectionStatusBorrowed, WifiSavedNetworkBorrowed};
use crate::contracts::WifiForgetContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiForgetProvider;

impl WifiForgetContract for VoidWifiForgetProvider {
    fn provide(
        &self,
        _network: WifiSavedNetworkBorrowed<'_>,
    ) -> Option<WifiConnectionStatusBorrowed<'_>> {
        None
    }
}
