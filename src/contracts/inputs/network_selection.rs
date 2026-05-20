use crate::borrowed_data::WifiNetworkSelectionInputView;

pub trait WifiNetworkSelectionInputContract {
    fn provide(&self) -> Option<WifiNetworkSelectionInputView<'_>>;
}
