#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiSavedSecretBorrowed<'a> {
    pub ssid: &'a str,
    pub network_id: &'a str,
    pub secret: &'a str,
    pub secret_kind: &'a str,
    pub source: &'a str,
}
