pub mod commands;
pub mod values;

use commands::{
    EvoWifiConnectCommand, EvoWifiDisconnectCommand, EvoWifiForgetCommand, EvoWifiNetworksCommand,
    EvoWifiStatusCommand,
};
use nu_plugin::{Plugin, PluginCommand};

pub struct EvoWifiPlugin;

impl Plugin for EvoWifiPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(EvoWifiStatusCommand),
            Box::new(EvoWifiNetworksCommand),
            Box::new(EvoWifiConnectCommand),
            Box::new(EvoWifiDisconnectCommand),
            Box::new(EvoWifiForgetCommand),
        ]
    }
}
