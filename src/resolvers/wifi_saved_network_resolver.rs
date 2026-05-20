use crate::borrowed::{
    WifiConnectionStatusBorrowed, WifiInterfaceBorrowed, WifiSavedNetworkBorrowed,
};
use crate::contracts::WifiSavedNetworkContract;

pub fn resolve<R>(
    provider: &impl WifiSavedNetworkContract,
    interface: WifiInterfaceBorrowed<'_>,
    status: WifiConnectionStatusBorrowed<'_>,
    next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(interface, status, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedSavedNetworkProvider;

    impl WifiSavedNetworkContract for ResolvedSavedNetworkProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _status: WifiConnectionStatusBorrowed<'_>,
            next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiSavedNetworkBorrowed {
                ssid: "example-wifi",
                network_id: "4",
            }))
        }
    }

    struct UnresolvedSavedNetworkProvider;

    impl WifiSavedNetworkContract for UnresolvedSavedNetworkProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _status: WifiConnectionStatusBorrowed<'_>,
            _next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_saved_network_from_provider() {
        let provider = ResolvedSavedNetworkProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let status = WifiConnectionStatusBorrowed {
            ssid: "example-wifi",
            status: "COMPLETED",
        };

        let result = resolve(&provider, interface, status, |network| {
            format!("{}:{}", network.network_id, network.ssid)
        });

        assert_eq!(result.as_deref(), Some("4:example-wifi"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_saved_network() {
        let provider = UnresolvedSavedNetworkProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let status = WifiConnectionStatusBorrowed {
            ssid: "example-wifi",
            status: "COMPLETED",
        };

        let result = resolve(&provider, interface, status, |_network| "should not run");

        assert_eq!(result, None);
    }
}
