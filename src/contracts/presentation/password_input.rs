use crate::borrowed_data::WifiPasswordInputView;

pub trait WifiPasswordInputPresentationContract {
    fn read_wifi_password(&self) -> Option<WifiPasswordInputView<'_>>;
}
