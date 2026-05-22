use crate::borrowed::{WifiConnectionState, WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiStatusContract;

#[derive(Clone, Copy, Debug)]
pub struct FakeWifiStatusProvider<'a> {
    ssid: &'a str,
    state: WifiConnectionState,
}

impl<'a> FakeWifiStatusProvider<'a> {
    pub fn new(ssid: &'a str, state: WifiConnectionState) -> Self {
        Self { ssid, state }
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
            state: self.state,
        }))
    }
}
