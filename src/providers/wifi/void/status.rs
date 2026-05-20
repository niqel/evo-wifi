use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiStatusContract;
use std::process::Command;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiStatusProvider;

impl WifiStatusContract for VoidWifiStatusProvider {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R> {
        let output = Command::new("wpa_cli")
            .arg("-i")
            .arg(interface.name)
            .arg("status")
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let raw = String::from_utf8(output.stdout).ok()?;
        let status = status_from_wpa_cli_output(&raw)?;

        Some(next(status))
    }
}

fn status_from_wpa_cli_output(raw: &str) -> Option<WifiConnectionStatusBorrowed<'_>> {
    let ssid = value_for_key(raw, "ssid").unwrap_or("");
    let status = value_for_key(raw, "wpa_state")?;

    Some(WifiConnectionStatusBorrowed { ssid, status })
}

fn value_for_key<'a>(raw: &'a str, key: &str) -> Option<&'a str> {
    raw.lines().find_map(|line| {
        let (candidate, value) = line.split_once('=')?;

        (candidate == key).then_some(value.trim())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_status_from_wpa_cli_output() {
        let raw = "bssid=00:11:22:33:44:55\nssid=evo-wifi\nwpa_state=COMPLETED\n";

        let status = status_from_wpa_cli_output(raw).expect("status should resolve");

        assert_eq!(
            status,
            WifiConnectionStatusBorrowed {
                ssid: "evo-wifi",
                status: "COMPLETED",
            }
        );
    }

    #[test]
    fn resolves_status_without_ssid() {
        let raw = "wpa_state=DISCONNECTED\n";

        let status = status_from_wpa_cli_output(raw).expect("status should resolve");

        assert_eq!(
            status,
            WifiConnectionStatusBorrowed {
                ssid: "",
                status: "DISCONNECTED",
            }
        );
    }

    #[test]
    fn rejects_output_without_wpa_state() {
        let raw = "ssid=evo-wifi\n";

        assert_eq!(status_from_wpa_cli_output(raw), None);
    }
}
