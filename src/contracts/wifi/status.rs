use crate::borrowed_data::{WifiConnectionStatusView, WifiInterfaceView};

pub trait WifiStatusContract {
    fn provide(&self, interface: WifiInterfaceView<'_>) -> Option<WifiConnectionStatusView<'_>>;
}
