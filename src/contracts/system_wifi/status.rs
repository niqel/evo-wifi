use crate::borrowed_data::{WifiConnectionStatusView, WifiInterfaceView};

pub trait WifiStatusSystemWifiContract {
    fn provide(&self, interface: WifiInterfaceView<'_>) -> Option<WifiConnectionStatusView<'_>>;
}
