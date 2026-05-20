use crate::borrowed_data::WifiActionSelectionInputView;

pub trait WifiActionSelectionInputContract {
    fn provide(&self) -> Option<WifiActionSelectionInputView<'_>>;
}
