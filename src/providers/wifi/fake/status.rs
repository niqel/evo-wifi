use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiStatusContract;

#[derive(Clone, Copy, Debug)]
pub struct FakeWifiStatusProvider<'a> {
    ssid: &'a str,
    status: &'a str,
}

impl<'a> FakeWifiStatusProvider<'a> {
    pub fn new(ssid: &'a str, status: &'a str) -> Self {
        Self { ssid, status }
    }
}

impl WifiStatusContract for FakeWifiStatusProvider<'_> {
    fn provide<R>(
        &self,
        _interface: WifiInterfaceBorrowed<'_>,
        next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R> {
        Some(next(WifiConnectionStatusBorrowed {
            ssid: self.ssid,
            status: self.status,
        }))
    }
}
