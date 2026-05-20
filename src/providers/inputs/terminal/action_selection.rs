use crate::borrowed::WifiActionSelectionInputBorrowed;
use crate::contracts::WifiActionSelectionInputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalActionSelectionInputProvider<'a> {
    raw: Option<&'a str>,
}

impl<'a> TerminalActionSelectionInputProvider<'a> {
    pub fn new(raw: Option<&'a str>) -> Self {
        Self { raw }
    }
}

impl WifiActionSelectionInputContract for TerminalActionSelectionInputProvider<'_> {
    fn provide(&self) -> Option<WifiActionSelectionInputBorrowed<'_>> {
        self.raw.map(|raw| WifiActionSelectionInputBorrowed { raw })
    }
}
