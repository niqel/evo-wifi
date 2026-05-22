use crate::contracts::{WifiInterfaceContract, WifiStatusContract, WifiStatusOutputContract};
use crate::resolvers::{
    wifi_connection_status_output_resolver, wifi_connection_status_resolver,
    wifi_interface_resolver,
};

pub fn show(
    interface_provider: &impl WifiInterfaceContract,
    status_provider: &impl WifiStatusContract,
    output_provider: &impl WifiStatusOutputContract,
) -> Option<()> {
    wifi_interface_resolver::resolve(interface_provider, |interface| {
        wifi_connection_status_resolver::resolve(status_provider, interface, |status| {
            wifi_connection_status_output_resolver::resolve(output_provider, status)
        })
    })
    .flatten()
}
