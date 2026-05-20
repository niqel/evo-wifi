use crate::agents::wifi_new_network_connector;
use crate::contracts::{
    WifiInterfaceContract, WifiMessageOutputContract, WifiNetworkSelectionInputContract,
    WifiNewNetworkConnectContract, WifiPasswordInputContract, WifiStatusOutputContract,
};

pub struct WifiNewNetworkConnectCommand<I, P, W, C, O, M> {
    input_provider: I,
    password_provider: P,
    interface_provider: W,
    connect_provider: C,
    status_output_provider: O,
    message_output_provider: M,
}

impl<I, P, W, C, O, M> WifiNewNetworkConnectCommand<I, P, W, C, O, M>
where
    I: WifiNetworkSelectionInputContract,
    P: WifiPasswordInputContract,
    W: WifiInterfaceContract,
    C: WifiNewNetworkConnectContract,
    O: WifiStatusOutputContract,
    M: WifiMessageOutputContract,
{
    pub fn new(
        input_provider: I,
        password_provider: P,
        interface_provider: W,
        connect_provider: C,
        status_output_provider: O,
        message_output_provider: M,
    ) -> Self {
        Self {
            input_provider,
            password_provider,
            interface_provider,
            connect_provider,
            status_output_provider,
            message_output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_new_network_connector::connect(
            &self.input_provider,
            &self.password_provider,
            &self.interface_provider,
            &self.connect_provider,
            &self.status_output_provider,
            &self.message_output_provider,
        )
    }
}
