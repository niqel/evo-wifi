use crate::borrowed_data::{WifiConnectionStatusView, WifiInterfaceView};

pub trait WifiStatusSystemWifiContract {
    fn resolve_wifi_connection_status(
        &self,
        interface: WifiInterfaceView<'_>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}
