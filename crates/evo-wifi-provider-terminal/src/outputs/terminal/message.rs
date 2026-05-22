use evo_wifi_core::contracts::WifiMessageOutputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalMessageOutputProvider;

impl WifiMessageOutputContract for TerminalMessageOutputProvider {
    fn provide(&self, message: &str) {
        println!("{message}");
    }
}
