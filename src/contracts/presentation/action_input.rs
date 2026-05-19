use crate::borrowed_data::WifiActionSelectionInputView;

pub trait WifiActionInputPresentationContract {
    fn provide(&self) -> Option<WifiActionSelectionInputView<'_>>;
}
