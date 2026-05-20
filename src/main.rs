fn main() {
    let action = std::env::args().nth(1);
    let selection = std::env::args().nth(2);
    let password = std::env::args().nth(3);

    match action.as_deref() {
        Some("already-connected") => {
            let command = evo_wifi::commands::WifiAlreadyConnectedNetworkHandleCommand::new(
                evo_wifi::providers::inputs::terminal::TerminalNetworkSelectionInputProvider::new(
                    selection.as_deref(),
                ),
                evo_wifi::providers::wifi::void::VoidWifiInterfaceProvider,
                evo_wifi::providers::wifi::void::VoidWifiStatusProvider,
                evo_wifi::providers::outputs::terminal::TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("connect") => {
            let command = evo_wifi::commands::WifiNetworkConnectCommand::new(
                evo_wifi::providers::inputs::terminal::TerminalNetworkSelectionInputProvider::new(
                    selection.as_deref(),
                ),
                evo_wifi::providers::wifi::void::VoidWifiInterfaceProvider,
                evo_wifi::providers::wifi::void::VoidWifiStatusProvider,
                evo_wifi::providers::wifi::void::VoidWifiSavedNetworkProvider,
                evo_wifi::providers::wifi::void::VoidWifiConnectProvider,
                evo_wifi::providers::outputs::terminal::TerminalStatusOutputProvider,
                evo_wifi::providers::outputs::terminal::TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("connect-new") => {
            let command = evo_wifi::commands::WifiNewNetworkConnectCommand::new(
                evo_wifi::providers::inputs::terminal::TerminalNetworkSelectionInputProvider::new(
                    selection.as_deref(),
                ),
                evo_wifi::providers::inputs::terminal::TerminalPasswordInputProvider::new(
                    password.as_deref(),
                ),
                evo_wifi::providers::wifi::void::VoidWifiInterfaceProvider,
                evo_wifi::providers::wifi::void::VoidWifiNewNetworkConnectProvider,
                evo_wifi::providers::outputs::terminal::TerminalStatusOutputProvider,
                evo_wifi::providers::outputs::terminal::TerminalMessageOutputProvider,
            );

            command.execute();
        }
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
            eprintln!(
                "usage: evo-wifi [status|networks|scan|password|secret|already-connected <ssid>|connect <ssid>|connect-new <ssid> <password>]"
            );
        }
    }
}
