use crate::borrowed::WifiInterfaceBorrowed;
use crate::contracts::WifiInterfaceContract;

#[derive(Clone, Copy, Debug)]
pub struct FakeWifiInterfaceProvider<'a> {
    name: &'a str,
}

impl<'a> FakeWifiInterfaceProvider<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl WifiInterfaceContract for FakeWifiInterfaceProvider<'_> {
    fn provide<R>(&self, next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R) -> Option<R> {
        Some(next(WifiInterfaceBorrowed { name: self.name }))
    }
}
