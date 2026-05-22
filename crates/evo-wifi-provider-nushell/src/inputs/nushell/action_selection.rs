use evo_wifi_core::borrowed::WifiActionSelectionInputBorrowed;
use evo_wifi_core::contracts::WifiActionSelectionInputContract;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NushellActionSelectionInputProvider {
    raw: Option<String>,
}

impl NushellActionSelectionInputProvider {
    pub fn new(raw: impl Into<Option<String>>) -> Self {
        Self { raw: raw.into() }
    }
}

impl WifiActionSelectionInputContract for NushellActionSelectionInputProvider {
    fn provide(&self) -> Option<WifiActionSelectionInputBorrowed<'_>> {
        self.raw
            .as_deref()
            .map(|raw| WifiActionSelectionInputBorrowed { raw })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_borrowed_action_from_owned_nushell_input() {
        let provider = NushellActionSelectionInputProvider::new(Some("connect".to_owned()));

        let action = provider.provide();

        assert_eq!(
            action,
            Some(WifiActionSelectionInputBorrowed { raw: "connect" })
        );
    }
}
