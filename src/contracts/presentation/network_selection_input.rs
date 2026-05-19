use crate::borrowed_data::WifiNetworkSelectionInputView;

pub trait WifiNetworkSelectionInputPresentationContract {
    fn provide(&self) -> Option<WifiNetworkSelectionInputView<'_>>;
}
