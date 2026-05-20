use crate::agents::wifi_connection_disconnector;
use crate::contracts::{
    WifiDisconnectContract, WifiInterfaceContract, WifiMessageOutputContract, WifiStatusContract,
    WifiStatusOutputContract,
};

pub struct WifiConnectionDisconnectCommand<I, S, D, O, M> {
    interface_provider: I,
    status_provider: S,
    disconnect_provider: D,
    status_output_provider: O,
    message_output_provider: M,
}

impl<I, S, D, O, M> WifiConnectionDisconnectCommand<I, S, D, O, M>
where
    I: WifiInterfaceContract,
    S: WifiStatusContract,
    D: WifiDisconnectContract,
    O: WifiStatusOutputContract,
    M: WifiMessageOutputContract,
{
    pub fn new(
        interface_provider: I,
        status_provider: S,
        disconnect_provider: D,
        status_output_provider: O,
        message_output_provider: M,
    ) -> Self {
        Self {
            interface_provider,
            status_provider,
            disconnect_provider,
            status_output_provider,
            message_output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_connection_disconnector::disconnect(
            &self.interface_provider,
            &self.status_provider,
            &self.disconnect_provider,
            &self.status_output_provider,
            &self.message_output_provider,
        )
    }
}
