use crate::contracts::WifiMessageOutputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalMessageOutputProvider;

impl WifiMessageOutputContract for TerminalMessageOutputProvider {
    fn provide(&self, message: &str) {
        println!("{message}");
    }
}
