use crate::borrowed::WifiNetworkSelectionInputBorrowed;

pub trait WifiNetworkSelectionInputContract {
    fn provide<R>(
        &self,
        next: impl FnOnce(WifiNetworkSelectionInputBorrowed<'_>) -> R,
    ) -> Option<R>;
}
