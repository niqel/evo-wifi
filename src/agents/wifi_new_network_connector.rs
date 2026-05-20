use crate::contracts::{
    WifiInterfaceContract, WifiMessageOutputContract, WifiNetworkSelectionInputContract,
    WifiNewNetworkConnectContract, WifiPasswordInputContract, WifiStatusOutputContract,
};
use crate::resolvers::{
    wifi_connection_status_output_resolver, wifi_interface_resolver, wifi_message_output_resolver,
    wifi_network_selection_input_resolver, wifi_new_network_connect_resolver,
    wifi_password_input_resolver,
};

pub fn connect(
    input_provider: &impl WifiNetworkSelectionInputContract,
    password_provider: &impl WifiPasswordInputContract,
    interface_provider: &impl WifiInterfaceContract,
    connect_provider: &impl WifiNewNetworkConnectContract,
    status_output_provider: &impl WifiStatusOutputContract,
    message_output_provider: &impl WifiMessageOutputContract,
) -> Option<()> {
    wifi_network_selection_input_resolver::resolve(input_provider, |selection| {
        wifi_password_input_resolver::resolve(password_provider, |password| {
            if password.raw.trim().is_empty() {
                return Some(wifi_message_output_resolver::resolve(
                    message_output_provider,
                    "Password is required for selected WiFi network",
                ));
            }

            wifi_interface_resolver::resolve(interface_provider, |interface| {
                wifi_new_network_connect_resolver::resolve(
                    connect_provider,
                    interface,
                    selection,
                    password,
                    |connected_status| {
                        wifi_connection_status_output_resolver::resolve(
                            status_output_provider,
                            connected_status,
                        )
                    },
                )
            })
            .flatten()
        })
    })
    .flatten()
    .flatten()
}
