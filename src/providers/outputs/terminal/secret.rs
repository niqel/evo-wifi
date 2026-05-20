use crate::borrowed::WifiSavedSecretBorrowed;
use crate::contracts::WifiSecretOutputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalSecretOutputProvider;

impl WifiSecretOutputContract for TerminalSecretOutputProvider {
    fn provide(&self, secret: WifiSavedSecretBorrowed<'_>) {
        println!(
            "{}\t{}\t{}\t{}\t{}",
            secret.ssid, secret.network_id, secret.secret, secret.secret_kind, secret.source
        );
    }
}
