use crate::borrowed::WifiNetworkBorrowed;
use crate::contracts::WifiNetworkOutputContract;

pub fn resolve(provider: &impl WifiNetworkOutputContract, networks: &[WifiNetworkBorrowed<'_>]) {
    provider.provide(networks);
}
