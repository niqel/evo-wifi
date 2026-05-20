use crate::borrowed_data::WifiSavedSecretView;

pub trait WifiSecretOutputContract {
    fn provide(&self, secret: WifiSavedSecretView<'_>);
}
