use crate::borrowed_data::{
    WifiConnectionStatusView, WifiNetworkSelectionInputView, WifiPasswordInputView,
};

pub trait WifiConnectSystemWifiContract {
    fn connect_to_wifi_network(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
        password: Option<WifiPasswordInputView<'_>>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}
