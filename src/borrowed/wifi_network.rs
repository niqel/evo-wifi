#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiNetworkBorrowed<'a> {
    pub bssid: &'a str,
    pub frequency_mhz: u32,
    pub signal_dbm: i32,
    pub flags: &'a str,
    pub ssid: &'a str,
}
