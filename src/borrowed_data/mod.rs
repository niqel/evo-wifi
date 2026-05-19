#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiInterfaceView<'a> {
    pub name: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiConnectionStatusView<'a> {
    pub ssid: &'a str,
    pub status: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiNetworkView<'a> {
    pub bssid: &'a str,
    pub frequency: &'a str,
    pub signal_dbm: &'a str,
    pub flags: &'a str,
    pub ssid: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiNetworkSelectionInputView<'a> {
    pub raw: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiActionSelectionInputView<'a> {
    pub raw: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiPasswordInputView<'a> {
    pub raw: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiSavedNetworkView<'a> {
    pub ssid: &'a str,
    pub network_id: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiSavedSecretView<'a> {
    pub ssid: &'a str,
    pub network_id: &'a str,
    pub secret: &'a str,
    pub secret_kind: &'a str,
    pub source: &'a str,
}
