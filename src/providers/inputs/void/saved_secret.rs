use crate::borrowed::{WifiInterfaceBorrowed, WifiSavedNetworkBorrowed, WifiSavedSecretBorrowed};
use crate::contracts::WifiSavedSecretContract;
use std::fs;
use std::process::Command;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiSavedSecretProvider;

impl WifiSavedSecretContract for VoidWifiSavedSecretProvider {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        network: WifiSavedNetworkBorrowed<'_>,
        next: impl FnOnce(WifiSavedSecretBorrowed<'_>) -> R,
    ) -> Option<R> {
        if let Some(raw) = wpa_cli_saved_secret(interface.name, network.network_id)
            && let Some(secret) = saved_secret_from_raw_wpa_secret(network, &raw, "wpa_cli")
        {
            return Some(next(secret));
        }

        for path in wpa_supplicant_config_paths(interface.name) {
            let raw = fs::read_to_string(&path).ok()?;

            if let Some(secret) = saved_secret_from_wpa_supplicant_config(network, &raw, &path) {
                return Some(next(secret));
            }
        }

        None
    }
}

fn wpa_cli_saved_secret(interface_name: &str, network_id: u32) -> Option<String> {
    let output = Command::new("wpa_cli")
        .arg("-i")
        .arg(interface_name)
        .arg("get_network")
        .arg(network_id.to_string())
        .arg("psk")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout).ok()
}

fn saved_secret_from_raw_wpa_secret<'a>(
    network: WifiSavedNetworkBorrowed<'a>,
    raw_secret: &'a str,
    source: &'a str,
) -> Option<WifiSavedSecretBorrowed<'a>> {
    let raw_secret = raw_secret.trim();

    if raw_secret.is_empty() || raw_secret == "FAIL" || raw_secret == "*" {
        return None;
    }

    let (secret, secret_kind) = normalized_secret(raw_secret);

    Some(WifiSavedSecretBorrowed {
        ssid: network.ssid,
        network_id: network.network_id,
        secret,
        secret_kind,
        source,
    })
}

fn saved_secret_from_wpa_supplicant_config<'a>(
    network: WifiSavedNetworkBorrowed<'a>,
    raw: &'a str,
    source: &'a str,
) -> Option<WifiSavedSecretBorrowed<'a>> {
    let mut in_network = false;
    let mut current_ssid = "";
    let mut current_psk = "";

    for line in raw.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("network={") {
            in_network = true;
            current_ssid = "";
            current_psk = "";
        } else if in_network && trimmed == "}" {
            if current_ssid == network.ssid && !current_psk.is_empty() {
                return saved_secret_from_raw_wpa_secret(network, current_psk, source);
            }

            in_network = false;
        } else if in_network {
            if let Some(ssid) = trimmed.strip_prefix("ssid=") {
                current_ssid = normalized_secret(ssid.trim()).0;
            } else if let Some(psk) = trimmed.strip_prefix("psk=") {
                current_psk = psk.trim();
            }
        }
    }

    None
}

fn normalized_secret(raw_secret: &str) -> (&str, &str) {
    if raw_secret.starts_with('"') && raw_secret.ends_with('"') && raw_secret.len() >= 2 {
        (&raw_secret[1..raw_secret.len() - 1], "password")
    } else if raw_secret.len() == 64 && raw_secret.chars().all(|c| c.is_ascii_hexdigit()) {
        (raw_secret, "psk_hex")
    } else {
        (raw_secret, "value")
    }
}

fn wpa_supplicant_config_paths(interface_name: &str) -> Vec<String> {
    [
        format!("/etc/wpa_supplicant/wpa_supplicant-{interface_name}.conf"),
        format!("/etc/wpa_supplicant/{interface_name}.conf"),
        "/etc/wpa_supplicant/wpa_supplicant.conf".to_owned(),
        "/etc/wpa_supplicant.conf".to_owned(),
    ]
    .into_iter()
    .filter(|path| fs::metadata(path).is_ok())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_password_secret_from_wpa_cli_output() {
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };

        let secret = saved_secret_from_raw_wpa_secret(network, "\"example-password\"\n", "wpa_cli")
            .expect("secret should resolve");

        assert_eq!(
            secret,
            WifiSavedSecretBorrowed {
                ssid: "example-wifi",
                network_id: 4,
                secret: "example-password",
                secret_kind: "password",
                source: "wpa_cli",
            }
        );
    }

    #[test]
    fn resolves_hex_psk_secret_from_wpa_cli_output() {
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };
        let raw_secret = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

        let secret = saved_secret_from_raw_wpa_secret(network, raw_secret, "wpa_cli")
            .expect("secret should resolve");

        assert_eq!(secret.secret, raw_secret);
        assert_eq!(secret.secret_kind, "psk_hex");
    }

    #[test]
    fn rejects_hidden_or_failed_wpa_cli_secret() {
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };

        assert_eq!(
            saved_secret_from_raw_wpa_secret(network, "*", "wpa_cli"),
            None
        );
        assert_eq!(
            saved_secret_from_raw_wpa_secret(network, "FAIL", "wpa_cli"),
            None
        );
    }

    #[test]
    fn resolves_secret_from_wpa_supplicant_config() {
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };
        let raw = r#"
network={
    ssid="example-wifi"
    psk="example-password"
}
"#;

        let secret = saved_secret_from_wpa_supplicant_config(network, raw, "config")
            .expect("secret should resolve");

        assert_eq!(
            secret,
            WifiSavedSecretBorrowed {
                ssid: "example-wifi",
                network_id: 4,
                secret: "example-password",
                secret_kind: "password",
                source: "config",
            }
        );
    }
}
