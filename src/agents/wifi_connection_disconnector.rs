use crate::contracts::{
    WifiDisconnectContract, WifiInterfaceContract, WifiMessageOutputContract, WifiStatusContract,
    WifiStatusOutputContract,
};
use crate::resolvers::{
    wifi_connection_disconnect_resolver, wifi_connection_status_output_resolver,
    wifi_connection_status_resolver, wifi_disconnect_message_resolver, wifi_interface_resolver,
    wifi_message_output_resolver,
};

pub fn disconnect(
    interface_provider: &impl WifiInterfaceContract,
    status_provider: &impl WifiStatusContract,
    disconnect_provider: &impl WifiDisconnectContract,
    status_output_provider: &impl WifiStatusOutputContract,
    message_output_provider: &impl WifiMessageOutputContract,
) -> Option<()> {
    wifi_interface_resolver::resolve(interface_provider, |interface| {
        wifi_connection_status_resolver::resolve(status_provider, interface, |status| {
            if let Some(message) = wifi_disconnect_message_resolver::resolve(status) {
                return Some(wifi_message_output_resolver::resolve(
                    message_output_provider,
                    message,
                ));
            }

            wifi_connection_disconnect_resolver::resolve(
                disconnect_provider,
                interface,
                |disconnected_status| {
                    wifi_connection_status_output_resolver::resolve(
                        status_output_provider,
                        disconnected_status,
                    )
                },
            )
        })
    })
    .flatten()
    .flatten()
}
