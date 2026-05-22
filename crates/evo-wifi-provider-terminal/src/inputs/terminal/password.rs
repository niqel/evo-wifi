use evo_wifi_core::borrowed::WifiPasswordInputBorrowed;
use evo_wifi_core::contracts::WifiPasswordInputContract;

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
    fn provide<R>(&self, next: impl FnOnce(WifiPasswordInputBorrowed<'_>) -> R) -> Option<R> {
        self.raw.map(|raw| next(WifiPasswordInputBorrowed { raw }))
    }
}
