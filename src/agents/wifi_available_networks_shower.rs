use crate::contracts::{WifiInterfaceContract, WifiNetworkOutputContract, WifiScanContract};
use crate::resolvers::{wifi_interface_resolver, wifi_network_output_resolver, wifi_scan_resolver};

pub fn show(
    interface_provider: &impl WifiInterfaceContract,
    scan_provider: &impl WifiScanContract,
    output_provider: &impl WifiNetworkOutputContract,
) -> Option<()> {
    wifi_interface_resolver::resolve(interface_provider, |interface| {
        wifi_scan_resolver::resolve(scan_provider, interface, |networks| {
            wifi_network_output_resolver::resolve(output_provider, networks)
        })
    })
    .flatten()
}
