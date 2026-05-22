use crate::borrowed::WifiConnectionStatusBorrowed;
use crate::contracts::WifiStatusOutputContract;

#[derive(Clone, Copy, Debug, Default)]
pub struct TerminalStatusOutputProvider;

impl WifiStatusOutputContract for TerminalStatusOutputProvider {
    fn provide(&self, status: WifiConnectionStatusBorrowed<'_>) {
        println!("{}\t{}", status.ssid, status.state.as_wpa_state());
    }
}
