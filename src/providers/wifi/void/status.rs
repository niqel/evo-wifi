use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiStatusContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiStatusProvider;

impl WifiStatusContract for VoidWifiStatusProvider {
    fn provide<R>(
        &self,
        _interface: WifiInterfaceBorrowed<'_>,
        _next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R> {
        None
    }
}
