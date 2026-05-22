use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};
use crate::contracts::WifiDisconnectContract;

pub fn resolve<R>(
    provider: &impl WifiDisconnectContract,
    interface: WifiInterfaceBorrowed<'_>,
    next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(interface, next)
}

#[cfg(test)]
mod tests {
    use crate::borrowed::WifiConnectionState;

    use super::*;

    struct ResolvedDisconnectProvider;

    impl WifiDisconnectContract for ResolvedDisconnectProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiConnectionStatusBorrowed {
                ssid: "",
                state: WifiConnectionState::Disconnected,
            }))
        }
    }

    struct UnresolvedDisconnectProvider;

    impl WifiDisconnectContract for UnresolvedDisconnectProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_disconnect_from_provider() {
        let provider = ResolvedDisconnectProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };

        let result = resolve(&provider, interface, |status| {
            format!("{}:{}", status.ssid, status.state.as_wpa_state())
        });

        assert_eq!(result.as_deref(), Some(":DISCONNECTED"));
    }

    #[test]
    fn returns_none_when_provider_cannot_disconnect() {
        let provider = UnresolvedDisconnectProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };

        let result = resolve(&provider, interface, |_status| "should not run");

        assert_eq!(result, None);
    }
}
