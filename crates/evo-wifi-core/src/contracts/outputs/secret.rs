use crate::borrowed::WifiSavedSecretBorrowed;

pub trait WifiSecretOutputContract {
    fn provide(&self, secret: WifiSavedSecretBorrowed<'_>);
}
