use crate::borrowed::WifiInterfaceBorrowed;
use crate::contracts::WifiInterfaceContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiInterfaceProvider;

impl WifiInterfaceContract for VoidWifiInterfaceProvider {
    fn provide<R>(&self, _next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R) -> Option<R> {
        None
    }
}
