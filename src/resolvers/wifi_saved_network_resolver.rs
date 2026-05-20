use crate::borrowed::{
    WifiInterfaceBorrowed, WifiNetworkSelectionInputBorrowed, WifiSavedNetworkBorrowed,
};
use crate::contracts::WifiSavedNetworkContract;

pub fn resolve<R>(
    provider: &impl WifiSavedNetworkContract,
    interface: WifiInterfaceBorrowed<'_>,
    selection: WifiNetworkSelectionInputBorrowed<'_>,
    next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(interface, selection, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedSavedNetworkProvider;

    impl WifiSavedNetworkContract for ResolvedSavedNetworkProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _selection: WifiNetworkSelectionInputBorrowed<'_>,
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
            _selection: WifiNetworkSelectionInputBorrowed<'_>,
            _next: impl FnOnce(WifiSavedNetworkBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_saved_network_from_provider() {
        let provider = ResolvedSavedNetworkProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };

        let result = resolve(&provider, interface, selection, |network| {
            format!("{}:{}", network.network_id, network.ssid)
        });

        assert_eq!(result.as_deref(), Some("4:example-wifi"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_saved_network() {
        let provider = UnresolvedSavedNetworkProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let selection = WifiNetworkSelectionInputBorrowed {
            raw: "example-wifi",
        };

        let result = resolve(&provider, interface, selection, |_network| "should not run");

        assert_eq!(result, None);
    }
}
