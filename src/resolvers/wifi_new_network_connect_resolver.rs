use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed,
    WifiPasswordInputBorrowed,
};
use crate::contracts::WifiNewNetworkConnectContract;

pub fn resolve<R>(
    provider: &impl WifiNewNetworkConnectContract,
    interface: WifiInterfaceBorrowed<'_>,
    selection: WifiNetworkSelectionInputBorrowed<'_>,
    password: WifiPasswordInputBorrowed<'_>,
    next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(interface, selection, password, next)
}

#[cfg(test)]
mod tests {
    use crate::borrowed::WifiConnectionState;

    use super::*;

    struct ResolvedNewNetworkConnectProvider;

    impl WifiNewNetworkConnectContract for ResolvedNewNetworkConnectProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            selection: WifiNetworkSelectionInputBorrowed<'_>,
            _password: WifiPasswordInputBorrowed<'_>,
            next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiConnectionStatusBorrowed {
                ssid: selection.raw,
                state: WifiConnectionState::Completed,
            }))
        }
    }

    struct UnresolvedNewNetworkConnectProvider;

    impl WifiNewNetworkConnectContract for UnresolvedNewNetworkConnectProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _selection: WifiNetworkSelectionInputBorrowed<'_>,
            _password: WifiPasswordInputBorrowed<'_>,
            _next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_new_network_connection_from_provider() {
        let provider = ResolvedNewNetworkConnectProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let password = WifiPasswordInputBorrowed {
            raw: "example-password",
        };

        let result = resolve(&provider, interface, selection, password, |status| {
            format!("{}:{}", status.ssid, status.state.as_wpa_state())
        });

        assert_eq!(result.as_deref(), Some("example-wifi:COMPLETED"));
    }

    #[test]
    fn returns_none_when_provider_cannot_connect_new_network() {
        let provider = UnresolvedNewNetworkConnectProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };
        let password = WifiPasswordInputBorrowed {
            raw: "example-password",
        };

        let result = resolve(
            &provider,
            interface,
            selection,
            password,
            |_status| "should not run",
        );

        assert_eq!(result, None);
    }
}
