use crate::borrowed_data::WifiInterfaceView;

pub trait WifiInterfaceSystemWifiContract {
    fn provide(&self) -> Option<WifiInterfaceView<'_>>;
}
