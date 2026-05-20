use crate::agents::wifi_connection_status_shower;
use crate::contracts::{WifiInterfaceContract, WifiStatusContract, WifiStatusOutputContract};

pub struct WifiConnectionStatusShowCommand<I, S, O> {
    interface_provider: I,
    status_provider: S,
    output_provider: O,
}

impl<I, S, O> WifiConnectionStatusShowCommand<I, S, O>
where
    I: WifiInterfaceContract,
    S: WifiStatusContract,
    O: WifiStatusOutputContract,
{
    pub fn new(interface_provider: I, status_provider: S, output_provider: O) -> Self {
        Self {
            interface_provider,
            status_provider,
            output_provider,
        }
    }

    pub fn execute(&self) -> Option<()> {
        wifi_connection_status_shower::show(
            &self.interface_provider,
            &self.status_provider,
            &self.output_provider,
        )
    }
}
