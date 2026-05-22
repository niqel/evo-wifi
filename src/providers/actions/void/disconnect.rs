use crate::borrowed::{WifiConnectionState, WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiDisconnectContract;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiDisconnectProvider;

impl WifiDisconnectContract for VoidWifiDisconnectProvider {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R> {
        let initial_raw = read_status(interface.name)?;
        let initial_status = status_from_wpa_cli_output(&initial_raw)?;

        ensure_wpa_cli_ok(
            Command::new("wpa_cli")
                .arg("-i")
                .arg(interface.name)
                .arg("disconnect")
                .output()
                .ok()?,
        )?;

        let raw = wait_disconnected_status(interface.name, 25)?;
        let status = status_from_wpa_cli_output(&raw)?;
        let status = WifiConnectionStatusBorrowed {
            ssid: if status.ssid.is_empty() {
                initial_status.ssid
            } else {
                status.ssid
            },
            state: status.state,
        };

        Some(next(status))
    }
}

fn ensure_wpa_cli_ok(output: Output) -> Option<()> {
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    (stdout.trim() == "OK").then_some(())
}

fn wait_disconnected_status(interface_name: &str, timeout_seconds: u64) -> Option<String> {
    for _ in 0..(timeout_seconds * 2) {
        let raw = read_status(interface_name)?;

        if let Some(status) = status_from_wpa_cli_output(&raw)
            && !status.state.is_completed()
        {
            return Some(raw);
        }

        thread::sleep(Duration::from_millis(500));
    }

    read_status(interface_name)
}

fn read_status(interface_name: &str) -> Option<String> {
    let output = Command::new("wpa_cli")
        .arg("-i")
        .arg(interface_name)
        .arg("status")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout).ok()
}

fn status_from_wpa_cli_output(raw: &str) -> Option<WifiConnectionStatusBorrowed<'_>> {
    let ssid = value_for_key(raw, "ssid").unwrap_or("");
    let status = value_for_key(raw, "wpa_state")?;

    Some(WifiConnectionStatusBorrowed {
        ssid,
        state: WifiConnectionState::from_wpa_state(status),
    })
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
        let raw = "ssid=example-wifi\nwpa_state=DISCONNECTED\n";

        let status = status_from_wpa_cli_output(raw).expect("status should resolve");

        assert_eq!(
            status,
            WifiConnectionStatusBorrowed {
                ssid: "example-wifi",
                state: WifiConnectionState::Disconnected,
            }
        );
    }

    #[test]
    fn rejects_output_without_wpa_state() {
        let raw = "ssid=example-wifi\n";

        assert_eq!(status_from_wpa_cli_output(raw), None);
    }
}
