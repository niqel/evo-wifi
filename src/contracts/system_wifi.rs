use crate::borrowed_data::{
    WifiConnectionStatusView, WifiInterfaceView, WifiNetworkSelectionInputView, WifiNetworkView,
    WifiPasswordInputView, WifiSavedNetworkView, WifiSavedSecretView,
};

pub trait WifiScanSystemWifiContract {
    fn scan_wifi_networks(&self) -> Option<&[WifiNetworkView<'_>]>;
}

pub trait WifiStatusSystemWifiContract {
    fn resolve_wifi_interface(&self) -> Option<WifiInterfaceView<'_>>;

    fn resolve_wifi_connection_status(
        &self,
        interface: WifiInterfaceView<'_>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}

pub trait WifiConnectSystemWifiContract {
    fn connect_to_wifi_network(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
        password: Option<WifiPasswordInputView<'_>>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}

pub trait WifiSavedNetworkSystemWifiContract {
    fn resolve_saved_wifi_network(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
    ) -> Option<WifiSavedNetworkView<'_>>;
}

pub trait WifiSavedSecretSystemWifiContract {
    fn resolve_saved_wifi_secret(
        &self,
        network: WifiSavedNetworkView<'_>,
    ) -> Option<WifiSavedSecretView<'_>>;
}

pub trait WifiForgetSystemWifiContract {
    fn forget_wifi_network(
        &self,
        network: WifiSavedNetworkView<'_>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}
