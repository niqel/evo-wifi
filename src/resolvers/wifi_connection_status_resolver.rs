use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiStatusContract;

pub fn resolve<R>(
    provider: &impl WifiStatusContract,
    interface: WifiInterfaceBorrowed<'_>,
    next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(interface, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedStatusProvider;

    impl WifiStatusContract for ResolvedStatusProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiConnectionStatusBorrowed {
                ssid: "evo-network",
                status: "connected",
            }))
        }
    }

    struct UnresolvedStatusProvider;

    impl WifiStatusContract for UnresolvedStatusProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_connection_status_from_provider() {
        let provider = ResolvedStatusProvider;
        let interface = WifiInterfaceBorrowed { name: "wlan0" };

        let result = resolve(&provider, interface, |status| {
            format!("{}:{}", status.ssid, status.status)
        });

        assert_eq!(result.as_deref(), Some("evo-network:connected"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_status() {
        let provider = UnresolvedStatusProvider;
        let interface = WifiInterfaceBorrowed { name: "wlan0" };

        let result = resolve(&provider, interface, |_status| "should not run");

        assert_eq!(result, None);
    }
}
