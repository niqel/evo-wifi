#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiSavedSecretBorrowed<'a> {
    pub ssid: &'a str,
    pub network_id: u32,
    pub secret: &'a str,
    pub secret_kind: &'a str,
    pub source: &'a str,
}
