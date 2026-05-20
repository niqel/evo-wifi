use crate::borrowed::WifiInterfaceBorrowed;
use crate::contracts::WifiInterfaceContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiInterfaceProvider;

impl WifiInterfaceContract for VoidWifiInterfaceProvider {
    fn provide(&self) -> Option<WifiInterfaceBorrowed<'_>> {
        None
    }
}
