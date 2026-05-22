use crate::borrowed::WifiInterfaceBorrowed;
use crate::contracts::WifiInterfaceContract;

pub fn resolve<R>(
    provider: &impl WifiInterfaceContract,
    next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedInterfaceProvider;

    impl WifiInterfaceContract for ResolvedInterfaceProvider {
        fn provide<R>(&self, next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R) -> Option<R> {
            let raw = "wlan0";
            Some(next(WifiInterfaceBorrowed { name: raw }))
        }
    }

    struct UnresolvedInterfaceProvider;

    impl WifiInterfaceContract for UnresolvedInterfaceProvider {
        fn provide<R>(&self, _next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_interface_from_provider() {
        let provider = ResolvedInterfaceProvider;

        let result = resolve(&provider, |interface| interface.name.to_owned());

        assert_eq!(result.as_deref(), Some("wlan0"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_interface() {
        let provider = UnresolvedInterfaceProvider;

        let result = resolve(&provider, |_interface| "should not run");

        assert_eq!(result, None);
    }
}
