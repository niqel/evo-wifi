use crate::borrowed::WifiPasswordInputBorrowed;
use crate::contracts::WifiPasswordInputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalPasswordInputProvider<'a> {
    raw: Option<&'a str>,
}

impl<'a> TerminalPasswordInputProvider<'a> {
    pub fn new(raw: Option<&'a str>) -> Self {
        Self { raw }
    }
}

impl WifiPasswordInputContract for TerminalPasswordInputProvider<'_> {
    fn provide(&self) -> Option<WifiPasswordInputBorrowed<'_>> {
        self.raw.map(|raw| WifiPasswordInputBorrowed { raw })
    }
}
