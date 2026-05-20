use crate::borrowed_data::WifiPasswordInputView;

pub trait WifiPasswordInputContract {
    fn provide(&self) -> Option<WifiPasswordInputView<'_>>;
}
