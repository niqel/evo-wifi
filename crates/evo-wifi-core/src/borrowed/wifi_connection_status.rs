#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiConnectionStatusBorrowed<'a> {
    pub ssid: &'a str,
    pub state: WifiConnectionState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WifiConnectionState {
    Completed,
    Disconnected,
    Other,
}

impl WifiConnectionState {
    pub fn from_wpa_state(raw: &str) -> Self {
        match raw {
            "COMPLETED" => Self::Completed,
            "DISCONNECTED" => Self::Disconnected,
            _ => Self::Other,
        }
    }

    pub fn as_wpa_state(self) -> &'static str {
        match self {
            Self::Completed => "COMPLETED",
            Self::Disconnected => "DISCONNECTED",
            Self::Other => "OTHER",
        }
    }

    pub fn is_completed(self) -> bool {
        self == Self::Completed
    }
}
