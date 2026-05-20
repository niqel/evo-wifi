use crate::borrowed::{WifiConnectionStatusBorrowed, WifiNetworkSelectionInputBorrowed};

pub fn resolve(
    selection: WifiNetworkSelectionInputBorrowed<'_>,
    status: WifiConnectionStatusBorrowed<'_>,
) -> Option<&'static str> {
    let selected = selection.raw.trim();

    if selected.is_empty() || selected != status.ssid || status.status != "COMPLETED" {
        return None;
    }

    Some("Already connected to selected WiFi network")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_message_when_selected_network_is_current_connection() {
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let status = WifiConnectionStatusBorrowed {
            ssid: "example-wifi",
            status: "COMPLETED",
        };

        assert_eq!(
            resolve(selection, status),
            Some("Already connected to selected WiFi network")
        );
    }

    #[test]
    fn returns_none_when_selected_network_is_different() {
        let selection = WifiNetworkSelectionInputBorrowed { raw: "other-wifi" };
        let status = WifiConnectionStatusBorrowed {
            ssid: "example-wifi",
            status: "COMPLETED",
        };

        assert_eq!(resolve(selection, status), None);
    }

    #[test]
    fn returns_none_when_current_status_is_not_connected() {
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let status = WifiConnectionStatusBorrowed {
            ssid: "example-wifi",
            status: "DISCONNECTED",
        };

        assert_eq!(resolve(selection, status), None);
    }
}
