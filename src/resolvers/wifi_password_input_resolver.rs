use crate::borrowed::WifiPasswordInputBorrowed;
use crate::contracts::WifiPasswordInputContract;

pub fn resolve<R>(
    provider: &impl WifiPasswordInputContract,
    next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R,
) -> Option<R> {
    provider.provide(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedPasswordInputProvider;

    impl WifiPasswordInputContract for ResolvedPasswordInputProvider {
        fn provide<R>(&self, next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R) -> Option<R> {
            Some(next(WifiPasswordInputBorrowed {
                raw: "example-password",
            }))
        }
    }

    struct UnresolvedPasswordInputProvider;

    impl WifiPasswordInputContract for UnresolvedPasswordInputProvider {
        fn provide<R>(&self, _next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_password_input_from_provider() {
        let provider = ResolvedPasswordInputProvider;

        let result = resolve(&provider, |password| password.raw.to_owned());

        assert_eq!(result.as_deref(), Some("example-password"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_password_input() {
        let provider = UnresolvedPasswordInputProvider;

        let result = resolve(&provider, |_password| "should not run");

        assert_eq!(result, None);
    }
}
