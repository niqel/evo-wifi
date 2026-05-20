use crate::borrowed::{WifiNetworkSelectionInputBorrowed, WifiSavedNetworkBorrowed};
use crate::contracts::WifiSavedNetworkContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiSavedNetworkProvider;

impl WifiSavedNetworkContract for VoidWifiSavedNetworkProvider {
    fn provide(
        &self,
        _selection: WifiNetworkSelectionInputBorrowed<'_>,
    ) -> Option<WifiSavedNetworkBorrowed<'_>> {
        None
    }
}
