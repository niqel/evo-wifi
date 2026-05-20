use crate::contracts::{
    WifiInterfaceContract, WifiMessageOutputContract, WifiNetworkSelectionInputContract,
    WifiStatusContract,
};
use crate::resolvers::{
    wifi_already_connected_network_resolver, wifi_connection_status_resolver,
    wifi_interface_resolver, wifi_message_output_resolver, wifi_network_selection_input_resolver,
};

pub fn handle(
    input_provider: &impl WifiNetworkSelectionInputContract,
    interface_provider: &impl WifiInterfaceContract,
    status_provider: &impl WifiStatusContract,
    output_provider: &impl WifiMessageOutputContract,
) -> Option<()> {
    wifi_network_selection_input_resolver::resolve(input_provider, |selection| {
        wifi_interface_resolver::resolve(interface_provider, |interface| {
            wifi_connection_status_resolver::resolve(status_provider, interface, |status| {
                wifi_already_connected_network_resolver::resolve(selection, status)
                    .map(|message| wifi_message_output_resolver::resolve(output_provider, message))
            })
        })
    })
    .flatten()
    .flatten()
    .flatten()
}
