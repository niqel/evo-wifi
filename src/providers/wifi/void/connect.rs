use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiNetworkSelectionInputBorrowed, WifiPasswordInputBorrowed,
};
use crate::contracts::WifiConnectContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiConnectProvider;

impl WifiConnectContract for VoidWifiConnectProvider {
    fn provide(
        &self,
        _selection: WifiNetworkSelectionInputBorrowed<'_>,
        _password: Option<WifiPasswordInputBorrowed<'_>>,
    ) -> Option<WifiConnectionStatusBorrowed<'_>> {
        None
    }
}
