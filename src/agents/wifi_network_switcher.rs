use crate::contracts::{
    WifiConnectContract, WifiInterfaceContract, WifiMessageOutputContract,
    WifiNetworkSelectionInputContract, WifiNewNetworkConnectContract, WifiPasswordInputContract,
    WifiSavedNetworkContract, WifiStatusContract, WifiStatusOutputContract,
};
use crate::resolvers::{
    wifi_connection_status_resolver, wifi_interface_resolver,
    wifi_network_selection_input_resolver, wifi_network_switch_resolver,
};

pub fn switch(
    selection_provider: &impl WifiNetworkSelectionInputContract,
    password_provider: &impl WifiPasswordInputContract,
    interface_provider: &impl WifiInterfaceContract,
    status_provider: &impl WifiStatusContract,
    saved_network_provider: &impl WifiSavedNetworkContract,
    connect_provider: &impl WifiConnectContract,
    new_connect_provider: &impl WifiNewNetworkConnectContract,
    status_output_provider: &impl WifiStatusOutputContract,
    message_output_provider: &impl WifiMessageOutputContract,
) -> Option<()> {
    wifi_network_selection_input_resolver::resolve(selection_provider, |selection| {
        wifi_interface_resolver::resolve(interface_provider, |interface| {
            wifi_connection_status_resolver::resolve(status_provider, interface, |status| {
                wifi_network_switch_resolver::resolve(
                    selection,
                    interface,
                    status,
                    password_provider,
                    saved_network_provider,
                    connect_provider,
                    new_connect_provider,
                    status_output_provider,
                    message_output_provider,
                )
            })
        })
    })
    .flatten()
    .flatten()
    .flatten()
}
