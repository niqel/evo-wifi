use crate::borrowed::WifiInterfaceBorrowed;
use crate::contracts::WifiInterfaceContract;
use std::env;
use std::fs;
use std::process::Command;

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiInterfaceProvider;

impl WifiInterfaceContract for VoidWifiInterfaceProvider {
    fn provide<R>(&self, next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R) -> Option<R> {
        if let Some(configured) = configured_interface() {
            return Some(next(WifiInterfaceBorrowed {
                name: configured.as_str(),
            }));
        }

        if let Some(raw) = command_output(Command::new("iw").arg("dev")) {
            if let Some(name) = first_iw_dev_interface_name(&raw) {
                return Some(next(WifiInterfaceBorrowed { name }));
            }
        }

        if let Some(raw) = command_output(Command::new("wpa_cli").arg("interface_list")) {
            if let Some(name) = first_wpa_cli_interface_name(&raw) {
                return Some(next(WifiInterfaceBorrowed { name }));
            }
        }

        if let Some(sys_interface) = first_sys_wireless_interface_name() {
            return Some(next(WifiInterfaceBorrowed {
                name: sys_interface.as_str(),
            }));
        }

        None
    }
}

fn configured_interface() -> Option<String> {
    let configured = env::var("EVO_WIFI_IFACE").ok()?;
    let configured = configured.trim();

    (!configured.is_empty()).then(|| configured.to_owned())
}

fn command_output(command: &mut Command) -> Option<String> {
    let output = command.output().ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout).ok()
}

fn first_iw_dev_interface_name(raw: &str) -> Option<&str> {
    raw.lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("Interface "))
        .map(str::trim)
        .filter(|name| !name.is_empty())
}

fn first_wpa_cli_interface_name(raw: &str) -> Option<&str> {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("Available interfaces"))
        .find_map(|line| selected_interface_name(line).or(Some(line)))
}

fn selected_interface_name(line: &str) -> Option<&str> {
    line.strip_prefix("Selected interface '")?
        .strip_suffix('\'')
        .filter(|name| !name.is_empty())
}

fn first_sys_wireless_interface_name() -> Option<String> {
    fs::read_dir("/sys/class/net")
        .ok()?
        .filter_map(Result::ok)
        .find_map(|entry| {
            let wireless_path = entry.path().join("wireless");

            if !wireless_path.exists() {
                return None;
            }

            entry.file_name().into_string().ok()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_first_interface_name_from_iw_dev_output() {
        let raw = "phy#0\n\tInterface wlp2s0\n\t\tifindex 3\n";

        assert_eq!(first_iw_dev_interface_name(raw), Some("wlp2s0"));
    }

    #[test]
    fn resolves_first_interface_name_from_wpa_cli_list_output() {
        let raw = "Available interfaces:\nwlan0\n";

        assert_eq!(first_wpa_cli_interface_name(raw), Some("wlan0"));
    }

    #[test]
    fn resolves_selected_interface_from_wpa_cli_output() {
        let raw = "Selected interface 'wlp2s0'\n";

        assert_eq!(first_wpa_cli_interface_name(raw), Some("wlp2s0"));
    }

    #[test]
    fn ignores_empty_interface_output() {
        let raw = "Available interfaces:\n\n";

        assert_eq!(first_wpa_cli_interface_name(raw), None);
    }
}
