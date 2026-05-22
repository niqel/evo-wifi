use crate::borrowed::WifiPasswordInputBorrowed;

pub trait WifiPasswordInputContract {
    fn provide<R>(&self, next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R) -> Option<R>;
}
