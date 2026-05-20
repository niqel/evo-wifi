use crate::borrowed::WifiPasswordInputBorrowed;

pub trait WifiPasswordInputContract {
    fn provide(&self) -> Option<WifiPasswordInputBorrowed<'_>>;
}
