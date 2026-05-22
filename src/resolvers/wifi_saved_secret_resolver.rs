use crate::borrowed::{WifiInterfaceBorrowed, WifiSavedNetworkBorrowed, WifiSavedSecretBorrowed};
use crate::contracts::WifiSavedSecretContract;

pub fn resolve<R>(
    provider: &impl WifiSavedSecretContract,
    interface: WifiInterfaceBorrowed<'_>,
    network: WifiSavedNetworkBorrowed<'_>,
    next: impl FnOnce(WifiSavedSecretBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(interface, network, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedSavedSecretProvider;

    impl WifiSavedSecretContract for ResolvedSavedSecretProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _network: WifiSavedNetworkBorrowed<'_>,
            next: impl FnOnce(WifiSavedSecretBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiSavedSecretBorrowed {
                ssid: "example-wifi",
                network_id: 4,
                secret: "example-secret",
                secret_kind: "password",
                source: "wpa_cli",
            }))
        }
    }

    struct UnresolvedSavedSecretProvider;

    impl WifiSavedSecretContract for UnresolvedSavedSecretProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _network: WifiSavedNetworkBorrowed<'_>,
            _next: impl FnOnce(WifiSavedSecretBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_saved_secret_from_provider() {
        let provider = ResolvedSavedSecretProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };

        let result = resolve(&provider, interface, network, |secret| {
            secret.secret.to_owned()
        });

        assert_eq!(result.as_deref(), Some("example-secret"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_saved_secret() {
        let provider = UnresolvedSavedSecretProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };
        let network = WifiSavedNetworkBorrowed {
            ssid: "example-wifi",
            network_id: 4,
        };

        let result = resolve(&provider, interface, network, |_secret| "should not run");

        assert_eq!(result, None);
    }
}
