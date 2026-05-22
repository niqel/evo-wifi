use crate::borrowed::WifiNetworkSelectionInputBorrowed;
use crate::contracts::{
    WifiInterfaceContract, WifiSavedNetworkContract, WifiSavedSecretContract,
    WifiSecretOutputContract, WifiStatusContract,
};
use crate::resolvers::{
    wifi_connection_status_resolver, wifi_interface_resolver, wifi_saved_network_resolver,
    wifi_saved_secret_output_resolver, wifi_saved_secret_resolver,
};

pub fn show(
    interface_provider: &impl WifiInterfaceContract,
    status_provider: &impl WifiStatusContract,
    saved_network_provider: &impl WifiSavedNetworkContract,
    saved_secret_provider: &impl WifiSavedSecretContract,
    output_provider: &impl WifiSecretOutputContract,
) -> Option<()> {
    wifi_interface_resolver::resolve(interface_provider, |interface| {
        wifi_connection_status_resolver::resolve(status_provider, interface, |status| {
            let selection = WifiNetworkSelectionInputBorrowed { raw: status.ssid };

            wifi_saved_network_resolver::resolve(
                saved_network_provider,
                interface,
                selection,
                |network| {
                    wifi_saved_secret_resolver::resolve(
                        saved_secret_provider,
                        interface,
                        network,
                        |secret| {
                            wifi_saved_secret_output_resolver::resolve(output_provider, secret)
                        },
                    )
                },
            )
        })
    })
    .flatten()
    .flatten()
    .flatten()
}
