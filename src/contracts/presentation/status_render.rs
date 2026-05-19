use crate::borrowed_data::WifiConnectionStatusView;

pub trait WifiStatusRenderPresentationContract {
    fn provide(&self, status: WifiConnectionStatusView<'_>);
}
