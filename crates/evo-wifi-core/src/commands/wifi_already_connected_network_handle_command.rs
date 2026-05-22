use crate::agents::wifi_already_connected_network_handler;
use crate::contracts::{
    WifiInterfaceContract, WifiMessageOutputContract, WifiNetworkSelectionInputContract,
    WifiStatusContract,
};

pub struct WifiAlreadyConnectedNetworkHandleCommand<I, W, S, O> {
    input_provider: I,
    interface_provider: W,
    status_provider: S,
    output_provider: O,
}

impl<I, W, S, O> WifiAlreadyConnectedNetworkHandleCommand<I, W, S, O>
where
    I: WifiNetworkSelectionInputContract,
    W: WifiInterfaceContract,
    S: WifiStatusContract,
    O: WifiMessageOutputContract,
{
    pub fn new(
        input_provider: I,
        interface_provider: W,
        status_provider: S,
        output_provider: O,
    ) -> Self {
        Self {
            input_provider,
            interface_provider,
            status_provider,
            output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_already_connected_network_handler::handle(
            &self.input_provider,
            &self.interface_provider,
            &self.status_provider,
            &self.output_provider,
        )
    }
}
