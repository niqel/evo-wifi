use evo_wifi_core::borrowed::WifiNetworkBorrowed;
use evo_wifi_core::contracts::WifiNetworkOutputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalNetworkOutputProvider;

impl WifiNetworkOutputContract for TerminalNetworkOutputProvider {
    fn provide(&self, networks: &[WifiNetworkBorrowed<'_>]) {
        if networks.is_empty() {
            println!("No WiFi networks found");
            return;
        }

        for network in networks {
            println!(
                "{}\t{}\t{}\t{}\t{}",
                network.bssid,
                network.frequency_mhz,
                network.signal_dbm,
                network.flags,
                network.ssid
            );
        }
    }
}
