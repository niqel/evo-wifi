use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiSavedNetworkBorrowed,
};
use crate::contracts::WifiConnectContract;

pub fn resolve<R>(
    provider: &impl WifiConnectContract,
    interface: WifiInterfaceBorrowed<'_>,
    network: WifiSavedNetworkBorrowed<'_>,
    next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(interface, network, next)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    struct UnresolvedConnectProvider;

    impl WifiConnectContract for UnresolvedConnectProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _network: WifiSavedNetworkBorrowed<'_>,
            _next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_connection_from_provider() {
        let provider = ResolvedConnectProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: "4",
        };

        let result = resolve(&provider, interface, network, |status| {
            format!("{}:{}", status.ssid, status.status)
        });

        assert_eq!(result.as_deref(), Some("example-wifi:COMPLETED"));
    }

    #[test]
    fn returns_none_when_provider_cannot_connect() {
        let provider = UnresolvedConnectProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: "4",
        };

        let result = resolve(&provider, interface, network, |_status| "should not run");

        assert_eq!(result, None);
    }
}
