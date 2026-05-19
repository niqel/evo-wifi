use crate::borrowed_data::{
    WifiConnectionStatusView, WifiNetworkSelectionInputView, WifiPasswordInputView,
};

pub trait WifiConnectSystemWifiContract {
    fn provide(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
        password: Option<WifiPasswordInputView<'_>>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}
