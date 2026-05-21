use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed,
};
use crate::contracts::{
    WifiConnectContract, WifiMessageOutputContract, WifiNewNetworkConnectContract,
    WifiPasswordInputContract, WifiSavedNetworkContract, WifiStatusOutputContract,
};
use crate::resolvers::{
    wifi_already_connected_network_resolver, wifi_connect_resolver,
    wifi_connection_status_output_resolver, wifi_message_output_resolver,
    wifi_new_network_connect_resolver, wifi_password_input_resolver, wifi_saved_network_resolver,
};

pub fn resolve(
    selection: WifiNetworkSelectionInputBorrowed<'_>,
    interface: WifiInterfaceBorrowed<'_>,
    status: WifiConnectionStatusBorrowed<'_>,
    password_provider: &impl WifiPasswordInputContract,
    saved_network_provider: &impl WifiSavedNetworkContract,
    connect_provider: &impl WifiConnectContract,
    new_connect_provider: &impl WifiNewNetworkConnectContract,
    status_output_provider: &impl WifiStatusOutputContract,
    message_output_provider: &impl WifiMessageOutputContract,
) -> Option<()> {
    if let Some(message) = wifi_already_connected_network_resolver::resolve(selection, status) {
        return Some(wifi_message_output_resolver::resolve(
            message_output_provider,
            message,
        ));
    }

    if let Some(result) = wifi_saved_network_resolver::resolve(
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
    ) {
        return result;
    }

    match wifi_password_input_resolver::resolve(password_provider, |password| {
        wifi_new_network_connect_resolver::resolve(
            new_connect_provider,
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
    }) {
        Some(result) => result,
        None => Some(wifi_message_output_resolver::resolve(
            message_output_provider,
            "Password required to connect to selected WiFi network",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::borrowed::{
        WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed,
        WifiPasswordInputBorrowed, WifiSavedNetworkBorrowed,
    };
    use crate::contracts::{
        WifiConnectContract, WifiMessageOutputContract, WifiNewNetworkConnectContract,
        WifiPasswordInputContract, WifiSavedNetworkContract, WifiStatusOutputContract,
    };
    use std::cell::RefCell;

    struct ResolvedSavedNetworkProvider;

    impl WifiSavedNetworkContract for ResolvedSavedNetworkProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            selection: WifiNetworkSelectionInputBorrowed<'_>,
            next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiSavedNetworkBorrowed {
                ssid: selection.raw,
                network_id: "4",
            }))
        }
    }

    struct MissingSavedNetworkProvider;

    impl WifiSavedNetworkContract for MissingSavedNetworkProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _selection: WifiNetworkSelectionInputBorrowed<'_>,
            _next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    struct ResolvedConnectProvider;

    impl WifiConnectContract for ResolvedConnectProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            network: WifiSavedNetworkBorrowed<'_>,
            next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiConnectionStatusBorrowed {
                ssid: network.ssid,
                status: "COMPLETED",
            }))
        }
    }

    struct ResolvedNewConnectProvider;

    impl WifiNewNetworkConnectContract for ResolvedNewConnectProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            selection: WifiNetworkSelectionInputBorrowed<'_>,
            _password: WifiPasswordInputBorrowed<'_>,
            next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiConnectionStatusBorrowed {
                ssid: selection.raw,
                status: "COMPLETED",
            }))
        }
    }

    struct CapturingStatusOutputProvider {
        captured: RefCell<Option<String>>,
    }

    impl WifiStatusOutputContract for CapturingStatusOutputProvider {
        fn provide(&self, status: WifiConnectionStatusBorrowed<'_>) {
            self.captured
                .borrow_mut()
                .replace(format!("{}\t{}", status.ssid, status.status));
        }
    }

    struct CapturingMessageOutputProvider {
        captured: RefCell<Option<String>>,
    }

    impl WifiMessageOutputContract for CapturingMessageOutputProvider {
        fn provide(&self, message: &str) {
            self.captured.borrow_mut().replace(message.to_owned());
        }
    }

    struct ResolvedPasswordProvider;

    impl WifiPasswordInputContract for ResolvedPasswordProvider {
        fn provide<R>(&self, next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R) -> Option<R> {
            Some(next(WifiPasswordInputBorrowed { raw: "secret" }))
        }
    }

    struct MissingPasswordProvider;

    impl WifiPasswordInputContract for MissingPasswordProvider {
        fn provide<R>(&self, _next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_already_connected_message() {
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let status = WifiConnectionStatusBorrowed {
            ssid: "example-wifi",
            status: "COMPLETED",
        };
        let message_output = CapturingMessageOutputProvider {
            captured: RefCell::new(None),
        };
        let status_output = CapturingStatusOutputProvider {
            captured: RefCell::new(None),
        };

        let result = resolve(
            selection,
            interface,
            status,
            &ResolvedPasswordProvider,
            &MissingSavedNetworkProvider,
            &ResolvedConnectProvider,
            &ResolvedNewConnectProvider,
            &status_output,
            &message_output,
        );

        assert_eq!(result, Some(()));
        assert_eq!(
            message_output.captured.borrow().as_deref(),
            Some("Already connected to selected WiFi network")
        );
        assert_eq!(status_output.captured.borrow().as_deref(), None);
    }

    #[test]
    fn resolves_saved_network_connect_path() {
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let status = WifiConnectionStatusBorrowed {
            ssid: "other-wifi",
            status: "COMPLETED",
        };
        let message_output = CapturingMessageOutputProvider {
            captured: RefCell::new(None),
        };
        let status_output = CapturingStatusOutputProvider {
            captured: RefCell::new(None),
        };

        let result = resolve(
            selection,
            interface,
            status,
            &MissingPasswordProvider,
            &ResolvedSavedNetworkProvider,
            &ResolvedConnectProvider,
            &ResolvedNewConnectProvider,
            &status_output,
            &message_output,
        );

        assert_eq!(result, Some(()));
        assert_eq!(
            status_output.captured.borrow().as_deref(),
            Some("example-wifi\tCOMPLETED")
        );
        assert_eq!(message_output.captured.borrow().as_deref(), None);
    }

    #[test]
    fn resolves_password_fallback_when_saved_network_is_missing() {
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let status = WifiConnectionStatusBorrowed {
            ssid: "other-wifi",
            status: "COMPLETED",
        };
        let message_output = CapturingMessageOutputProvider {
            captured: RefCell::new(None),
        };
        let status_output = CapturingStatusOutputProvider {
            captured: RefCell::new(None),
        };

        let result = resolve(
            selection,
            interface,
            status,
            &ResolvedPasswordProvider,
            &MissingSavedNetworkProvider,
            &ResolvedConnectProvider,
            &ResolvedNewConnectProvider,
            &status_output,
            &message_output,
        );

        assert_eq!(result, Some(()));
        assert_eq!(
            status_output.captured.borrow().as_deref(),
            Some("example-wifi\tCOMPLETED")
        );
        assert_eq!(message_output.captured.borrow().as_deref(), None);
    }

    #[test]
    fn resolves_missing_password_message_when_new_connection_needs_input() {
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let status = WifiConnectionStatusBorrowed {
            ssid: "other-wifi",
            status: "COMPLETED",
        };
        let message_output = CapturingMessageOutputProvider {
            captured: RefCell::new(None),
        };
        let status_output = CapturingStatusOutputProvider {
            captured: RefCell::new(None),
        };

        let result = resolve(
            selection,
            interface,
            status,
            &MissingPasswordProvider,
            &MissingSavedNetworkProvider,
            &ResolvedConnectProvider,
            &ResolvedNewConnectProvider,
            &status_output,
            &message_output,
        );

        assert_eq!(result, Some(()));
        assert_eq!(
            message_output.captured.borrow().as_deref(),
            Some("Password required to connect to selected WiFi network")
        );
        assert_eq!(status_output.captured.borrow().as_deref(), None);
    }
}
