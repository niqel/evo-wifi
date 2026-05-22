use crate::contracts::WifiMessageOutputContract;

pub fn resolve(provider: &impl WifiMessageOutputContract, message: &str) {
    provider.provide(message);
}
