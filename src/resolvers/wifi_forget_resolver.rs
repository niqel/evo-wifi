use crate::borrowed::{WifiInterfaceBorrowed, WifiSavedNetworkBorrowed};
use crate::contracts::WifiForgetContract;

pub fn resolve<R>(
    provider: &impl WifiForgetContract,
    interface: WifiInterfaceBorrowed<'_>,
    network: WifiSavedNetworkBorrowed<'_>,
    next: impl FnOnce() -> R,
) -> Option<R> {
    provider.provide(interface, network, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedForgetProvider;

    impl WifiForgetContract for ResolvedForgetProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _network: WifiSavedNetworkBorrowed<'_>,
            next: impl FnOnce() -> R,
        ) -> Option<R> {
            Some(next())
        }
    }

    struct UnresolvedForgetProvider;

    impl WifiForgetContract for UnresolvedForgetProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _network: WifiSavedNetworkBorrowed<'_>,
            _next: impl FnOnce() -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_forget_from_provider() {
        let provider = ResolvedForgetProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };

        let result = resolve(&provider, interface, network, || "forgotten");

        assert_eq!(result, Some("forgotten"));
    }

    #[test]
    fn returns_none_when_provider_cannot_forget() {
        let provider = UnresolvedForgetProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };

        let result = resolve(&provider, interface, network, || "should not run");

        assert_eq!(result, None);
    }
}
