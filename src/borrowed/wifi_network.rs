#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiNetworkBorrowed<'a> {
    pub bssid: &'a str,
    pub frequency: &'a str,
    pub signal_dbm: &'a str,
    pub flags: &'a str,
    pub ssid: &'a str,
}
