use crate::borrowed_data::WifiInterfaceView;

pub trait WifiInterfaceSystemWifiContract {
    fn resolve_wifi_interface(&self) -> Option<WifiInterfaceView<'_>>;
}
