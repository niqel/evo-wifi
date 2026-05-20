use crate::borrowed_data::{
    WifiConnectionStatusView, WifiNetworkSelectionInputView, WifiPasswordInputView,
};

pub trait WifiConnectContract {
    fn provide(
        &self,
        selection: WifiNetworkSelectionInputView<'_>,
        password: Option<WifiPasswordInputView<'_>>,
    ) -> Option<WifiConnectionStatusView<'_>>;
}
