use crate::borrowed_data::WifiActionSelectionInputView;

pub trait WifiActionInputPresentationContract {
    fn read_wifi_action_selection(&self) -> Option<WifiActionSelectionInputView<'_>>;
}
