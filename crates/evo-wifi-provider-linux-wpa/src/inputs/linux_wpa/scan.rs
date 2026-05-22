use evo_wifi_core::borrowed::{WifiInterfaceBorrowed, WifiNetworkBorrowed};
use evo_wifi_core::contracts::WifiScanContract;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct LinuxWpaWifiScanProvider;

impl WifiScanContract for LinuxWpaWifiScanProvider {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        next: impl FnOnce(&[WifiNetworkBorrowed<'_>]) -> R,
    ) -> Option<R> {
        let scan = Command::new("wpa_cli")
            .arg("-i")
            .arg(interface.name)
            .arg("scan")
            .output()
            .ok()?;

        if !scan.status.success() {
            return None;
        }

        thread::sleep(Duration::from_secs(2));

        let output = Command::new("wpa_cli")
            .arg("-i")
            .arg(interface.name)
            .arg("scan_results")
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let raw = String::from_utf8(output.stdout).ok()?;
        let networks = networks_from_wpa_cli_scan_results(&raw);

        Some(next(&networks))
    }
}

fn networks_from_wpa_cli_scan_results(raw: &str) -> Vec<WifiNetworkBorrowed<'_>> {
    raw.lines()
        .skip(1)
        .filter_map(network_from_wpa_cli_scan_result_line)
        .collect()
}

fn network_from_wpa_cli_scan_result_line(line: &str) -> Option<WifiNetworkBorrowed<'_>> {
    let mut cells = line.splitn(5, '\t');
    let bssid = cells.next()?.trim();
    let frequency = cells.next()?.trim();
    let signal_dbm = cells.next()?.trim();
    let flags = cells.next()?.trim();
    let ssid = cells.next()?.trim();

    if bssid.is_empty() || frequency.is_empty() || signal_dbm.is_empty() || flags.is_empty() {
        return None;
    }

    let frequency_mhz = frequency.parse().ok()?;
    let signal_dbm = signal_dbm.parse().ok()?;

    Some(WifiNetworkBorrowed {
        bssid,
        frequency_mhz,
        signal_dbm,
        flags,
        ssid: if ssid.is_empty() { "<hidden>" } else { ssid },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_networks_from_wpa_cli_scan_results() {
        let raw = "\
bssid / frequency / signal level / flags / ssid
aa:bb:cc:dd:ee:ff\t5200\t-51\t[WPA2-PSK-CCMP][ESS]\texample-wifi-5g
00:11:22:33:44:55\t2412\t-70\t[ESS]\t
";

        let networks = networks_from_wpa_cli_scan_results(raw);

        assert_eq!(
            networks,
            [
                WifiNetworkBorrowed {
                    bssid: "aa:bb:cc:dd:ee:ff",
                    frequency_mhz: 5200,
                    signal_dbm: -51,
                    flags: "[WPA2-PSK-CCMP][ESS]",
                    ssid: "example-wifi-5g",
                },
                WifiNetworkBorrowed {
                    bssid: "00:11:22:33:44:55",
                    frequency_mhz: 2412,
                    signal_dbm: -70,
                    flags: "[ESS]",
                    ssid: "<hidden>",
                },
            ]
        );
    }

    #[test]
    fn skips_malformed_scan_result_lines() {
        let raw = "\
bssid / frequency / signal level / flags / ssid
not-enough-columns
";

        let networks = networks_from_wpa_cli_scan_results(raw);

        assert!(networks.is_empty());
    }
}
