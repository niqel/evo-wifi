#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiConnectionStatusBorrowed<'a> {
    pub ssid: &'a str,
    pub status: &'a str,
}
