fn main() {
    let action = std::env::args().nth(1);

    match action.as_deref() {
        Some("networks" | "scan") => {
            let command = evo_wifi::commands::WifiAvailableNetworksShowCommand::new(
                evo_wifi::providers::wifi::void::VoidWifiInterfaceProvider,
                evo_wifi::providers::wifi::void::VoidWifiScanProvider,
                evo_wifi::providers::outputs::terminal::TerminalNetworkOutputProvider,
            );

            command.execute();
        }
        Some("password" | "secret") => {
            let command = evo_wifi::commands::WifiCurrentConnectionPasswordShowCommand::new(
                evo_wifi::providers::wifi::void::VoidWifiInterfaceProvider,
                evo_wifi::providers::wifi::void::VoidWifiStatusProvider,
                evo_wifi::providers::wifi::void::VoidWifiSavedNetworkProvider,
                evo_wifi::providers::wifi::void::VoidWifiSavedSecretProvider,
                evo_wifi::providers::outputs::terminal::TerminalSecretOutputProvider,
            );

            command.execute();
        }
        Some("status") | None => {
            let command = evo_wifi::commands::WifiConnectionStatusShowCommand::new(
                evo_wifi::providers::wifi::void::VoidWifiInterfaceProvider,
                evo_wifi::providers::wifi::void::VoidWifiStatusProvider,
                evo_wifi::providers::outputs::terminal::TerminalStatusOutputProvider,
            );

            command.execute();
        }
        Some(_) => {
            eprintln!("usage: evo-wifi [status|networks|scan|password|secret]");
        }
    }
}
