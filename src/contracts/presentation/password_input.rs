use crate::borrowed_data::WifiPasswordInputView;

pub trait WifiPasswordInputPresentationContract {
    fn provide(&self) -> Option<WifiPasswordInputView<'_>>;
}
