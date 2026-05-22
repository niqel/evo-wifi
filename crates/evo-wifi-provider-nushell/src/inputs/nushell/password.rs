use evo_wifi_core::borrowed::WifiPasswordInputBorrowed;
use evo_wifi_core::contracts::WifiPasswordInputContract;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NushellPasswordInputProvider {
    raw: Option<String>,
}

impl NushellPasswordInputProvider {
    pub fn new(raw: impl Into<Option<String>>) -> Self {
        Self { raw: raw.into() }
    }
}

impl WifiPasswordInputContract for NushellPasswordInputProvider {
    fn provide<R>(&self, next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R) -> Option<R> {
        self.raw
            .as_deref()
            .map(|raw| next(WifiPasswordInputBorrowed { raw }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_borrowed_password_from_owned_nushell_input() {
        let provider = NushellPasswordInputProvider::new(Some("secret".to_owned()));

        let password = provider.provide(|password| password.raw.to_owned());

        assert_eq!(password, Some("secret".to_owned()));
    }
}
