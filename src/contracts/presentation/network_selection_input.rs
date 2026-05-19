use crate::borrowed_data::WifiNetworkSelectionInputView;

pub trait WifiNetworkSelectionInputPresentationContract {
    fn read_wifi_network_selection(&self) -> Option<WifiNetworkSelectionInputView<'_>>;
}
