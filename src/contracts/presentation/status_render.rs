use crate::borrowed_data::WifiConnectionStatusView;

pub trait WifiStatusRenderPresentationContract {
    fn render_wifi_connection_status(&self, status: WifiConnectionStatusView<'_>);

    fn render_wifi_connection_status_message(&self, message: &str);
}
