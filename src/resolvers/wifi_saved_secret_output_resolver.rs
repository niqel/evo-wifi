use crate::borrowed::WifiSavedSecretBorrowed;
use crate::contracts::WifiSecretOutputContract;

pub fn resolve(provider: &impl WifiSecretOutputContract, secret: WifiSavedSecretBorrowed<'_>) {
    provider.provide(secret);
}
