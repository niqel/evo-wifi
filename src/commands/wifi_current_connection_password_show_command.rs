use crate::agents::wifi_current_connection_password_shower;
use crate::contracts::{
    WifiInterfaceContract, WifiSavedNetworkContract, WifiSavedSecretContract,
    WifiSecretOutputContract, WifiStatusContract,
};

pub struct WifiCurrentConnectionPasswordShowCommand<I, S, N, P, O> {
    interface_provider: I,
    status_provider: S,
    saved_network_provider: N,
    saved_secret_provider: P,
    output_provider: O,
}

impl<I, S, N, P, O> WifiCurrentConnectionPasswordShowCommand<I, S, N, P, O>
where
    I: WifiInterfaceContract,
    S: WifiStatusContract,
    N: WifiSavedNetworkContract,
    P: WifiSavedSecretContract,
    O: WifiSecretOutputContract,
{
    pub fn new(
        interface_provider: I,
        status_provider: S,
        saved_network_provider: N,
        saved_secret_provider: P,
        output_provider: O,
    ) -> Self {
        Self {
            interface_provider,
            status_provider,
            saved_network_provider,
            saved_secret_provider,
            output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_current_connection_password_shower::show(
            &self.interface_provider,
            &self.status_provider,
            &self.saved_network_provider,
            &self.saved_secret_provider,
            &self.output_provider,
        )
    }
}
