use crate::borrowed::WifiConnectionStatusBorrowed;

pub fn resolve(status: WifiConnectionStatusBorrowed<'_>) -> Option<&'static str> {
    if status.status == "COMPLETED" {
        None
    } else {
        Some("No active WiFi connection to disconnect")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_message_when_status_is_not_connected() {
        let status = WifiConnectionStatusBorrowed {
            ssid: "",
            status: "DISCONNECTED",
        };

        assert_eq!(
            resolve(status),
            Some("No active WiFi connection to disconnect")
        );
    }

    #[test]
    fn returns_none_when_status_is_connected() {
        let status = WifiConnectionStatusBorrowed {
            ssid: "example-wifi",
            status: "COMPLETED",
        };

        assert_eq!(resolve(status), None);
    }
}
