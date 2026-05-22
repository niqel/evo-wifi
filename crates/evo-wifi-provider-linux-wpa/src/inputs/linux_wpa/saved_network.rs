use evo_wifi_core::borrowed::{
    WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed, WifiSavedNetworkBorrowed,
};
use evo_wifi_core::contracts::WifiSavedNetworkContract;
use std::process::Command;

#[derive(Clone, Copy, Debug, Default)]
pub struct LinuxWpaWifiSavedNetworkProvider;

impl WifiSavedNetworkContract for LinuxWpaWifiSavedNetworkProvider {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        selection: WifiNetworkSelectionInputBorrowed<'_>,
        next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
    ) -> Option<R> {
        let ssid = selection.raw.trim();

        if ssid.is_empty() {
            return None;
        }

        let output = Command::new("wpa_cli")
            .arg("-i")
            .arg(interface.name)
            .arg("list_networks")
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let raw = String::from_utf8(output.stdout).ok()?;
        let network = saved_network_from_wpa_cli_list_networks(&raw, ssid)?;

        Some(next(network))
    }
}

fn saved_network_from_wpa_cli_list_networks<'a>(
    raw: &'a str,
    ssid: &str,
) -> Option<WifiSavedNetworkBorrowed<'a>> {
    raw.lines()
        .skip(1)
        .filter_map(saved_network_from_wpa_cli_list_networks_line)
        .find(|network| network.ssid == ssid)
}

fn saved_network_from_wpa_cli_list_networks_line(
    line: &str,
) -> Option<WifiSavedNetworkBorrowed<'_>> {
    let mut cells = line.split('\t');
    let network_id = cells.next()?.trim();
    let ssid = cells.next()?.trim();

    if network_id.is_empty() || ssid.is_empty() {
        return None;
    }

    let network_id = network_id.parse().ok()?;

    Some(WifiSavedNetworkBorrowed { ssid, network_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_saved_network_from_wpa_cli_list_networks() {
        let raw = "\
network id / ssid / bssid / flags
0\texample-wifi\tany\t[CURRENT]
1\tother-wifi\tany\t[DISABLED]
";

        let network = saved_network_from_wpa_cli_list_networks(raw, "example-wifi")
            .expect("network should resolve");

        assert_eq!(
            network,
            WifiSavedNetworkBorrowed {
                ssid: "example-wifi",
                network_id: 0,
            }
        );
    }

    #[test]
    fn returns_none_when_saved_network_is_missing() {
        let raw = "\
network id / ssid / bssid / flags
1\tother-wifi\tany\t[DISABLED]
";

        assert_eq!(
            saved_network_from_wpa_cli_list_networks(raw, "example-wifi"),
            None
        );
    }
}
