use crate::borrowed::WifiNetworkSelectionInputBorrowed;
use crate::contracts::WifiNetworkSelectionInputContract;

pub fn resolve<R>(
    provider: &impl WifiNetworkSelectionInputContract,
    next: impl FnOnce(WifiNetworkSelectionInputBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedNetworkSelectionInputProvider;

    impl WifiNetworkSelectionInputContract for ResolvedNetworkSelectionInputProvider {
        fn provide<R>(
            &self,
            next: impl FnOnce(WifiNetworkSelectionInputBorrowed<'_>) -> R,
        ) -> Option<R> {
            Some(next(WifiNetworkSelectionInputBorrowed {
                raw: "example-wifi",
            }))
        }
    }

    struct UnresolvedNetworkSelectionInputProvider;

    impl WifiNetworkSelectionInputContract for UnresolvedNetworkSelectionInputProvider {
        fn provide<R>(
            &self,
            _next: impl FnOnce(WifiNetworkSelectionInputBorrowed<'_>) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_network_selection_input_from_provider() {
        let provider = ResolvedNetworkSelectionInputProvider;

        let result = resolve(&provider, |selection| selection.raw.to_owned());

        assert_eq!(result.as_deref(), Some("example-wifi"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_selection_input() {
        let provider = UnresolvedNetworkSelectionInputProvider;

        let result = resolve(&provider, |_selection| "should not run");

        assert_eq!(result, None);
    }
}
