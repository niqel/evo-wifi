use crate::borrowed::WifiConnectionStatusBorrowed;

pub trait WifiStatusOutputContract {
    fn provide(&self, status: WifiConnectionStatusBorrowed<'_>);
}
