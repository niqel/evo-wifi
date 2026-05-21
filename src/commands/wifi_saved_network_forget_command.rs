use crate::agents::wifi_saved_network_forgetter;
use crate::contracts::{
    WifiForgetContract, WifiInterfaceContract, WifiMessageOutputContract,
    WifiNetworkSelectionInputContract, WifiSavedNetworkContract,
};

pub struct WifiSavedNetworkForgetCommand<I, W, S, F, M> {
    input_provider: I,
    interface_provider: W,
    saved_network_provider: S,
    forget_provider: F,
    message_output_provider: M,
}

impl<I, W, S, F, M> WifiSavedNetworkForgetCommand<I, W, S, F, M>
where
    I: WifiNetworkSelectionInputContract,
    W: WifiInterfaceContract,
    S: WifiSavedNetworkContract,
    F: WifiForgetContract,
    M: WifiMessageOutputContract,
{
    pub fn new(
        input_provider: I,
        interface_provider: W,
        saved_network_provider: S,
        forget_provider: F,
        message_output_provider: M,
    ) -> Self {
        Self {
            input_provider,
            interface_provider,
            saved_network_provider,
            forget_provider,
            message_output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_saved_network_forgetter::forget(
            &self.input_provider,
            &self.interface_provider,
            &self.saved_network_provider,
            &self.forget_provider,
            &self.message_output_provider,
        )
    }
}
