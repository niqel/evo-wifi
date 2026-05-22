use crate::agents::wifi_network_switcher;
use crate::contracts::{
    WifiConnectContract, WifiInterfaceContract, WifiMessageOutputContract,
    WifiNetworkSelectionInputContract, WifiNewNetworkConnectContract, WifiPasswordInputContract,
    WifiSavedNetworkContract, WifiStatusContract, WifiStatusOutputContract,
};

pub struct WifiNetworkSwitchCommand<I, P, W, S, N, C, NC, O, M> {
    selection_provider: I,
    password_provider: P,
    interface_provider: W,
    status_provider: S,
    saved_network_provider: N,
    connect_provider: C,
    new_connect_provider: NC,
    status_output_provider: O,
    message_output_provider: M,
}

impl<I, P, W, S, N, C, NC, O, M> WifiNetworkSwitchCommand<I, P, W, S, N, C, NC, O, M>
where
    I: WifiNetworkSelectionInputContract,
    P: WifiPasswordInputContract,
    W: WifiInterfaceContract,
    S: WifiStatusContract,
    N: WifiSavedNetworkContract,
    C: WifiConnectContract,
    NC: WifiNewNetworkConnectContract,
    O: WifiStatusOutputContract,
    M: WifiMessageOutputContract,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        selection_provider: I,
        password_provider: P,
        interface_provider: W,
        status_provider: S,
        saved_network_provider: N,
        connect_provider: C,
        new_connect_provider: NC,
        status_output_provider: O,
        message_output_provider: M,
    ) -> Self {
        Self {
            selection_provider,
            password_provider,
            interface_provider,
            status_provider,
            saved_network_provider,
            connect_provider,
            new_connect_provider,
            status_output_provider,
            message_output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_network_switcher::switch(
            &self.selection_provider,
            &self.password_provider,
            &self.interface_provider,
            &self.status_provider,
            &self.saved_network_provider,
            &self.connect_provider,
            &self.new_connect_provider,
            &self.status_output_provider,
            &self.message_output_provider,
        )
    }
}
