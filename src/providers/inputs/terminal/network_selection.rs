use crate::borrowed::WifiNetworkSelectionInputBorrowed;
use crate::contracts::WifiNetworkSelectionInputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalNetworkSelectionInputProvider<'a> {
    raw: Option<&'a str>,
}

impl<'a> TerminalNetworkSelectionInputProvider<'a> {
    pub fn new(raw: Option<&'a str>) -> Self {
        Self { raw }
    }
}

impl WifiNetworkSelectionInputContract for TerminalNetworkSelectionInputProvider<'_> {
    fn provide(&self) -> Option<WifiNetworkSelectionInputBorrowed<'_>> {
        self.raw
            .map(|raw| WifiNetworkSelectionInputBorrowed { raw })
    }
}
