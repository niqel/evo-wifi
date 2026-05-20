use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiStatusContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiStatusProvider;

impl WifiStatusContract for VoidWifiStatusProvider {
    fn provide(
        &self,
        _interface: WifiInterfaceBorrowed<'_>,
    ) -> Option<WifiConnectionStatusBorrowed<'_>> {
        None
    }
}
