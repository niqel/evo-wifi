fn main() {
    let command = evo_wifi::commands::WifiConnectionStatusShowCommand::new(
        evo_wifi::providers::wifi::fake::FakeWifiInterfaceProvider::new("wlan0"),
        evo_wifi::providers::wifi::fake::FakeWifiStatusProvider::new("evo-wifi", "connected"),
        evo_wifi::providers::outputs::terminal::TerminalStatusOutputProvider,
    );

    command.execute();
}
