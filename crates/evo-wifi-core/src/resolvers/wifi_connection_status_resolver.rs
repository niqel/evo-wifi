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
    use crate::borrowed::WifiConnectionState;

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
                state: WifiConnectionState::Completed,
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
            format!("{}:{}", status.ssid, status.state.as_wpa_state())
        });

        assert_eq!(result.as_deref(), Some("evo-network:COMPLETED"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_status() {
        let provider = UnresolvedStatusProvider;
        let interface = WifiInterfaceBorrowed { name: "wlan0" };

        let result = resolve(&provider, interface, |_status| "should not run");

        assert_eq!(result, None);
    }
}
