use crate::borrowed::WifiNetworkSelectionInputBorrowed;

pub trait WifiNetworkSelectionInputContract {
    fn provide(&self) -> Option<WifiNetworkSelectionInputBorrowed<'_>>;
}
