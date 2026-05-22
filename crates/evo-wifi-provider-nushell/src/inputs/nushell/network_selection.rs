use evo_wifi_core::borrowed::WifiNetworkSelectionInputBorrowed;
use evo_wifi_core::contracts::WifiNetworkSelectionInputContract;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NushellNetworkSelectionInputProvider {
    raw: Option<String>,
}

impl NushellNetworkSelectionInputProvider {
    pub fn new(raw: impl Into<Option<String>>) -> Self {
        Self { raw: raw.into() }
    }
}

impl WifiNetworkSelectionInputContract for NushellNetworkSelectionInputProvider {
    fn provide<R>(
        &self,
        next: impl FnOnce(WifiNetworkSelectionInputBorrowed<'_>) -> R,
    ) -> Option<R> {
        self.raw
            .as_deref()
            .map(|raw| next(WifiNetworkSelectionInputBorrowed { raw }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_borrowed_selection_from_owned_nushell_input() {
        let provider = NushellNetworkSelectionInputProvider::new(Some("evo-wifi".to_owned()));

        let selection = provider.provide(|selection| selection.raw.to_owned());

        assert_eq!(selection, Some("evo-wifi".to_owned()));
    }

    #[test]
    fn returns_none_when_selection_is_missing() {
        let provider = NushellNetworkSelectionInputProvider::default();

        let selection = provider.provide(|selection| selection.raw.to_owned());

        assert_eq!(selection, None);
    }
}
