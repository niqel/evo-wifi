#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiSavedNetworkBorrowed<'a> {
    pub ssid: &'a str,
    pub network_id: u32,
}
