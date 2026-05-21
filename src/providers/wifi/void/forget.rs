use crate::borrowed::{WifiInterfaceBorrowed, WifiSavedNetworkBorrowed};
use crate::contracts::WifiForgetContract;
use std::process::{Command, Output};

#[derive(Clone, Copy, Debug, Default)]
pub struct VoidWifiForgetProvider;

impl WifiForgetContract for VoidWifiForgetProvider {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        network: WifiSavedNetworkBorrowed<'_>,
        next: impl FnOnce() -> R,
    ) -> Option<R> {
        ensure_wpa_cli_ok(
            Command::new("wpa_cli")
                .arg("-i")
                .arg(interface.name)
                .arg("remove_network")
                .arg(network.network_id)
                .output()
                .ok()?,
        )?;

        let _ = Command::new("wpa_cli")
            .arg("-i")
            .arg(interface.name)
            .arg("save_config")
            .output();

        Some(next())
    }
}

fn ensure_wpa_cli_ok(output: Output) -> Option<()> {
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    (stdout.trim() == "OK").then_some(())
}
