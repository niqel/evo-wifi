use crate::borrowed::WifiConnectionStatusBorrowed;
use crate::contracts::WifiStatusOutputContract;

pub fn resolve(provider: &impl WifiStatusOutputContract, status: WifiConnectionStatusBorrowed<'_>) {
    provider.provide(status);
}
