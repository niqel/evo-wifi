use crate::contracts::{
    WifiConnectContract, WifiInterfaceContract, WifiMessageOutputContract,
    WifiNetworkSelectionInputContract, WifiSavedNetworkContract, WifiStatusContract,
    WifiStatusOutputContract,
};
use crate::resolvers::{
    wifi_already_connected_network_resolver, wifi_connect_resolver,
    wifi_connection_status_output_resolver, wifi_connection_status_resolver,
    wifi_interface_resolver, wifi_message_output_resolver, wifi_network_selection_input_resolver,
    wifi_saved_network_resolver,
};

pub fn connect(
    input_provider: &impl WifiNetworkSelectionInputContract,
    interface_provider: &impl WifiInterfaceContract,
    status_provider: &impl WifiStatusContract,
    saved_network_provider: &impl WifiSavedNetworkContract,
    connect_provider: &impl WifiConnectContract,
    status_output_provider: &impl WifiStatusOutputContract,
    message_output_provider: &impl WifiMessageOutputContract,
) -> Option<()> {
    wifi_network_selection_input_resolver::resolve(input_provider, |selection| {
        wifi_interface_resolver::resolve(interface_provider, |interface| {
            wifi_connection_status_resolver::resolve(status_provider, interface, |status| {
                if let Some(message) =
                    wifi_already_connected_network_resolver::resolve(selection, status)
                {
                    wifi_message_output_resolver::resolve(message_output_provider, message);

                    return Some(());
                }

                wifi_saved_network_resolver::resolve(
                    saved_network_provider,
                    interface,
                    selection,
                    |network| {
                        wifi_connect_resolver::resolve(
                            connect_provider,
                            interface,
                            network,
                            |connected_status| {
                                wifi_connection_status_output_resolver::resolve(
                                    status_output_provider,
                                    connected_status,
                                )
                            },
                        )
                    },
                )
                .flatten()
            })
        })
    })
    .flatten()
    .flatten()
    .flatten()
}
