use crate::borrowed::WifiNetworkBorrowed;

pub trait WifiNetworkOutputContract {
    fn provide(&self, networks: &[WifiNetworkBorrowed<'_>]);
}
