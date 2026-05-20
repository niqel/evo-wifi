use crate::agents::wifi_network_connector;
use crate::contracts::{
    WifiConnectContract, WifiInterfaceContract, WifiMessageOutputContract,
    WifiNetworkSelectionInputContract, WifiSavedNetworkContract, WifiStatusContract,
    WifiStatusOutputContract,
};

pub struct WifiNetworkConnectCommand<I, W, S, N, C, O, M> {
    input_provider: I,
    interface_provider: W,
    status_provider: S,
    saved_network_provider: N,
    connect_provider: C,
    status_output_provider: O,
    message_output_provider: M,
}

impl<I, W, S, N, C, O, M> WifiNetworkConnectCommand<I, W, S, N, C, O, M>
where
    I: WifiNetworkSelectionInputContract,
    W: WifiInterfaceContract,
    S: WifiStatusContract,
    N: WifiSavedNetworkContract,
    C: WifiConnectContract,
    O: WifiStatusOutputContract,
    M: WifiMessageOutputContract,
{
    pub fn new(
        input_provider: I,
        interface_provider: W,
        status_provider: S,
        saved_network_provider: N,
        connect_provider: C,
        status_output_provider: O,
        message_output_provider: M,
    ) -> Self {
        Self {
            input_provider,
            interface_provider,
            status_provider,
            saved_network_provider,
            connect_provider,
            status_output_provider,
            message_output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_network_connector::connect(
            &self.input_provider,
            &self.interface_provider,
            &self.status_provider,
            &self.saved_network_provider,
            &self.connect_provider,
            &self.status_output_provider,
            &self.message_output_provider,
        )
    }
}
