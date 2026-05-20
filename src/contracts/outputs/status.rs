use crate::borrowed_data::WifiConnectionStatusView;

pub trait WifiStatusOutputContract {
    fn provide(&self, status: WifiConnectionStatusView<'_>);
}
