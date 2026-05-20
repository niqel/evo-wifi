use crate::borrowed_data::WifiInterfaceView;

pub trait WifiInterfaceContract {
    fn provide(&self) -> Option<WifiInterfaceView<'_>>;
}
