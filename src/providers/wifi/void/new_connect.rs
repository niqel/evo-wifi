use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed,
    WifiPasswordInputBorrowed,
};
use crate::contracts::WifiNewNetworkConnectContract;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiNewNetworkConnectProvider;

impl WifiNewNetworkConnectContract for VoidWifiNewNetworkConnectProvider {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        selection: WifiNetworkSelectionInputBorrowed<'_>,
        password: WifiPasswordInputBorrowed<'_>,
        next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R> {
        let ssid = selection.raw.trim();
        let password = password.raw.trim();

        if ssid.is_empty() || password.is_empty() {
            return None;
        }

        let network_id = add_network(interface.name)?;

        if configure_network(interface.name, network_id.as_str(), ssid, password).is_none() {
            let _ = remove_network(interface.name, network_id.as_str());
            return None;
        }

        if connect_network(interface.name, network_id.as_str()).is_none() {
            let _ = remove_network(interface.name, network_id.as_str());
            return None;
        }

        let raw = wait_connected_status(interface.name, ssid, 25)?;
        let status = status_from_wpa_cli_output(&raw)?;

        if status.status != "COMPLETED" || status.ssid != ssid {
            let _ = remove_network(interface.name, network_id.as_str());
            return None;
        }

        if !raw.lines().any(|line| line.starts_with("ip_address=")) {
            let _ = Command::new("dhcpcd")
                .arg("-n")
                .arg(interface.name)
                .output();
        }

        ensure_wpa_cli_ok(
            Command::new("wpa_cli")
                .arg("-i")
                .arg(interface.name)
                .arg("save_config")
                .output()
                .ok()?,
        )?;

        Some(next(status))
    }
}

fn add_network(interface_name: &str) -> Option<String> {
    let output = Command::new("wpa_cli")
        .arg("-i")
        .arg(interface_name)
        .arg("add_network")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let network_id = String::from_utf8(output.stdout).ok()?;
    let network_id = network_id.trim();

    if network_id.is_empty() || !network_id.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    Some(network_id.to_owned())
}

fn configure_network(
    interface_name: &str,
    network_id: &str,
    ssid: &str,
    password: &str,
) -> Option<()> {
    ensure_wpa_cli_ok(
        Command::new("wpa_cli")
            .arg("-i")
            .arg(interface_name)
            .arg("set_network")
            .arg(network_id)
            .arg("ssid")
            .arg(wpa_quote(ssid))
            .output()
            .ok()?,
    )?;

    ensure_wpa_cli_ok(
        Command::new("wpa_cli")
            .arg("-i")
            .arg(interface_name)
            .arg("set_network")
            .arg(network_id)
            .arg("psk")
            .arg(wpa_psk_value(password))
            .output()
            .ok()?,
    )
}

fn connect_network(interface_name: &str, network_id: &str) -> Option<()> {
    ensure_wpa_cli_ok(
        Command::new("wpa_cli")
            .arg("-i")
            .arg(interface_name)
            .arg("enable_network")
            .arg(network_id)
            .output()
            .ok()?,
    )?;

    ensure_wpa_cli_ok(
        Command::new("wpa_cli")
            .arg("-i")
            .arg(interface_name)
            .arg("select_network")
            .arg(network_id)
            .output()
            .ok()?,
    )?;

    let _ = Command::new("wpa_cli")
        .arg("-i")
        .arg(interface_name)
        .arg("reassociate")
        .output();

    Some(())
}

fn remove_network(interface_name: &str, network_id: &str) -> Option<()> {
    ensure_wpa_cli_ok(
        Command::new("wpa_cli")
            .arg("-i")
            .arg(interface_name)
            .arg("remove_network")
            .arg(network_id)
            .output()
            .ok()?,
    )
}

fn ensure_wpa_cli_ok(output: Output) -> Option<()> {
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    (stdout.trim() == "OK").then_some(())
}

fn wait_connected_status(interface_name: &str, ssid: &str, timeout_seconds: u64) -> Option<String> {
    for _ in 0..(timeout_seconds * 2) {
        let raw = read_status(interface_name)?;

        if let Some(status) = status_from_wpa_cli_output(&raw) {
            if status.status == "COMPLETED" && status.ssid == ssid {
                return Some(raw);
            }
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

    Some(WifiConnectionStatusBorrowed { ssid, status })
}

fn value_for_key<'a>(raw: &'a str, key: &str) -> Option<&'a str> {
    raw.lines().find_map(|line| {
        let (candidate, value) = line.split_once('=')?;

        (candidate == key).then_some(value.trim())
    })
}

fn wpa_quote(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");

    format!("\"{escaped}\"")
}

fn wpa_psk_value(value: &str) -> String {
    if value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit()) {
        value.to_owned()
    } else {
        wpa_quote(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_wpa_values() {
        assert_eq!(wpa_quote("example"), "\"example\"");
        assert_eq!(wpa_quote("exa\"mple"), "\"exa\\\"mple\"");
    }

    #[test]
    fn keeps_hex_psk_unquoted() {
        let psk = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

        assert_eq!(wpa_psk_value(psk), psk);
    }

    #[test]
    fn quotes_plain_password_psk() {
        assert_eq!(wpa_psk_value("example-password"), "\"example-password\"");
    }

    #[test]
    fn resolves_status_from_wpa_cli_output() {
        let raw = "ssid=example-wifi\nwpa_state=COMPLETED\n";

        let status = status_from_wpa_cli_output(raw).expect("status should resolve");

        assert_eq!(
            status,
            WifiConnectionStatusBorrowed {
                ssid: "example-wifi",
                status: "COMPLETED",
            }
        );
    }
}
