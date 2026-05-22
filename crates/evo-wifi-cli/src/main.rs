use evo_wifi_core::commands::{
    WifiAlreadyConnectedNetworkHandleCommand, WifiAvailableNetworksShowCommand,
    WifiConnectionDisconnectCommand, WifiConnectionStatusShowCommand,
    WifiCurrentConnectionPasswordShowCommand, WifiNetworkConnectCommand, WifiNetworkSwitchCommand,
    WifiNewNetworkConnectCommand, WifiSavedNetworkForgetCommand,
};
use evo_wifi_provider_linux_wpa::actions::linux_wpa::{
    LinuxWpaWifiConnectProvider, LinuxWpaWifiDisconnectProvider, LinuxWpaWifiForgetProvider,
    LinuxWpaWifiNewNetworkConnectProvider,
};
use evo_wifi_provider_linux_wpa::inputs::linux_wpa::{
    LinuxWpaWifiInterfaceProvider, LinuxWpaWifiSavedNetworkProvider,
    LinuxWpaWifiSavedSecretProvider, LinuxWpaWifiScanProvider, LinuxWpaWifiStatusProvider,
};
use evo_wifi_provider_terminal::inputs::terminal::{
    TerminalNetworkSelectionInputProvider, TerminalPasswordInputProvider,
};
use evo_wifi_provider_terminal::outputs::terminal::{
    TerminalMessageOutputProvider, TerminalNetworkOutputProvider, TerminalSecretOutputProvider,
    TerminalStatusOutputProvider,
};

fn main() {
    let action = std::env::args().nth(1);
    let selection = std::env::args().nth(2);
    let password = std::env::args().nth(3);

    match action.as_deref() {
        Some("already-connected") => {
            let command = WifiAlreadyConnectedNetworkHandleCommand::new(
                TerminalNetworkSelectionInputProvider::new(selection.as_deref()),
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiStatusProvider,
                TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("connect") => {
            let command = WifiNetworkConnectCommand::new(
                TerminalNetworkSelectionInputProvider::new(selection.as_deref()),
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiStatusProvider,
                LinuxWpaWifiSavedNetworkProvider,
                LinuxWpaWifiConnectProvider,
                TerminalStatusOutputProvider,
                TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("connect-new") => {
            let command = WifiNewNetworkConnectCommand::new(
                TerminalNetworkSelectionInputProvider::new(selection.as_deref()),
                TerminalPasswordInputProvider::new(password.as_deref()),
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiNewNetworkConnectProvider,
                TerminalStatusOutputProvider,
                TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("switch") => {
            let command = WifiNetworkSwitchCommand::new(
                TerminalNetworkSelectionInputProvider::new(selection.as_deref()),
                TerminalPasswordInputProvider::new(password.as_deref()),
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiStatusProvider,
                LinuxWpaWifiSavedNetworkProvider,
                LinuxWpaWifiConnectProvider,
                LinuxWpaWifiNewNetworkConnectProvider,
                TerminalStatusOutputProvider,
                TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("forget") => {
            let command = WifiSavedNetworkForgetCommand::new(
                TerminalNetworkSelectionInputProvider::new(selection.as_deref()),
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiSavedNetworkProvider,
                LinuxWpaWifiForgetProvider,
                TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("disconnect") => {
            let command = WifiConnectionDisconnectCommand::new(
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiStatusProvider,
                LinuxWpaWifiDisconnectProvider,
                TerminalStatusOutputProvider,
                TerminalMessageOutputProvider,
            );

            command.execute();
        }
        Some("networks" | "scan") => {
            let command = WifiAvailableNetworksShowCommand::new(
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiScanProvider,
                TerminalNetworkOutputProvider,
            );

            command.execute();
        }
        Some("password" | "secret") => {
            let command = WifiCurrentConnectionPasswordShowCommand::new(
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiStatusProvider,
                LinuxWpaWifiSavedNetworkProvider,
                LinuxWpaWifiSavedSecretProvider,
                TerminalSecretOutputProvider,
            );

            command.execute();
        }
        Some("status") | None => {
            let command = WifiConnectionStatusShowCommand::new(
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiStatusProvider,
                TerminalStatusOutputProvider,
            );

            command.execute();
        }
        Some(_) => {
            eprintln!(
                "usage: evo-wifi [status|switch <ssid> [password]|forget <ssid>|disconnect|networks|scan|password|secret|already-connected <ssid>|connect <ssid>|connect-new <ssid> <password>]"
            );
        }
    }
}
