use crate::agents::wifi_available_networks_shower;
use crate::contracts::{WifiInterfaceContract, WifiNetworkOutputContract, WifiScanContract};

pub struct WifiAvailableNetworksShowCommand<I, S, O> {
    interface_provider: I,
    scan_provider: S,
    output_provider: O,
}

impl<I, S, O> WifiAvailableNetworksShowCommand<I, S, O>
where
    I: WifiInterfaceContract,
    S: WifiScanContract,
    O: WifiNetworkOutputContract,
{
    pub fn new(interface_provider: I, scan_provider: S, output_provider: O) -> Self {
        Self {
            interface_provider,
            scan_provider,
            output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_available_networks_shower::show(
            &self.interface_provider,
            &self.scan_provider,
            &self.output_provider,
        )
    }
}
