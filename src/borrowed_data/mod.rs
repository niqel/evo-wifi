pub struct WifiInterfaceView<'a> {
    pub name: &'a str,
}

pub struct WifiConnectionStatusView<'a> {
    pub ssid: &'a str,
    pub status: &'a str,
}
