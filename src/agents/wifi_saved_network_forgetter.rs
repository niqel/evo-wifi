use crate::contracts::{
    WifiForgetContract, WifiInterfaceContract, WifiMessageOutputContract,
    WifiNetworkSelectionInputContract, WifiSavedNetworkContract,
};
use crate::resolvers::{
    wifi_forget_resolver, wifi_interface_resolver, wifi_message_output_resolver,
    wifi_network_selection_input_resolver, wifi_saved_network_resolver,
};

pub fn forget(
    input_provider: &impl WifiNetworkSelectionInputContract,
    interface_provider: &impl WifiInterfaceContract,
    saved_network_provider: &impl WifiSavedNetworkContract,
    forget_provider: &impl WifiForgetContract,
    message_output_provider: &impl WifiMessageOutputContract,
) -> Option<()> {
    wifi_network_selection_input_resolver::resolve(input_provider, |selection| {
        wifi_interface_resolver::resolve(interface_provider, |interface| {
            wifi_saved_network_resolver::resolve(
                saved_network_provider,
                interface,
                selection,
                |network| match wifi_forget_resolver::resolve(forget_provider, interface, network, || {
                    wifi_message_output_resolver::resolve(
                        message_output_provider,
                        "Forgot saved WiFi password",
                    )
                }) {
                    Some(result) => Some(result),
                    None => Some(wifi_message_output_resolver::resolve(
                        message_output_provider,
                        "Saved WiFi password could not be removed",
                    )),
                },
            )
            .unwrap_or_else(|| {
                Some(wifi_message_output_resolver::resolve(
                    message_output_provider,
                    "No saved WiFi password to remove",
                ))
            })
        })
    })
    .flatten()
    .flatten()
}
