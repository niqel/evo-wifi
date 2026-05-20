fn main() {
    let command = evo_wifi::commands::WifiConnectionStatusShowCommand::new(
        evo_wifi::providers::wifi::void::VoidWifiInterfaceProvider,
        evo_wifi::providers::wifi::void::VoidWifiStatusProvider,
        evo_wifi::providers::outputs::terminal::TerminalStatusOutputProvider,
    );

    command.execute();
}
