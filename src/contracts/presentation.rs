use crate::borrowed_data::{
    WifiActionSelectionInputView, WifiConnectionStatusView, WifiNetworkSelectionInputView,
    WifiNetworkView, WifiPasswordInputView, WifiSavedSecretView,
};

pub trait WifiActionInputPresentationContract {
    fn read_wifi_action_selection(&self) -> Option<WifiActionSelectionInputView<'_>>;
}

pub trait WifiNetworkSelectionInputPresentationContract {
    fn read_wifi_network_selection(&self) -> Option<WifiNetworkSelectionInputView<'_>>;
}

pub trait WifiPasswordInputPresentationContract {
    fn read_wifi_password(&self) -> Option<WifiPasswordInputView<'_>>;
}

pub trait WifiNetworkRenderPresentationContract {
    fn render_wifi_networks(&self, networks: &[WifiNetworkView<'_>]);
}

pub trait WifiStatusRenderPresentationContract {
    fn render_wifi_connection_status(&self, status: WifiConnectionStatusView<'_>);

    fn render_wifi_connection_status_message(&self, message: &str);
}

pub trait WifiSecretRenderPresentationContract {
    fn render_wifi_secret(&self, secret: WifiSavedSecretView<'_>);
}

pub trait WifiMessageRenderPresentationContract {
    fn render_wifi_message(&self, message: &str);
}
