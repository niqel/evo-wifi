use crate::EvoWifiPlugin;
use crate::commands::common::return_output_value;
use evo_wifi_core::commands::{WifiNetworkConnectCommand, WifiNewNetworkConnectCommand};
use evo_wifi_provider_linux_wpa::actions::linux_wpa::{
    LinuxWpaWifiConnectProvider, LinuxWpaWifiNewNetworkConnectProvider,
};
use evo_wifi_provider_linux_wpa::inputs::linux_wpa::{
    LinuxWpaWifiInterfaceProvider, LinuxWpaWifiSavedNetworkProvider, LinuxWpaWifiStatusProvider,
};
use evo_wifi_provider_nushell::inputs::nushell::{
    NushellNetworkSelectionInputProvider, NushellPasswordInputProvider,
};
use evo_wifi_provider_nushell::outputs::nushell::{
    NushellMessageOutputProvider, NushellStatusOutputProvider,
};
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};

pub struct EvoWifiConnectCommand;

impl SimplePluginCommand for EvoWifiConnectCommand {
    type Plugin = EvoWifiPlugin;

    fn name(&self) -> &str {
        "evo-nu-wifi connect"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required_named("ssid", SyntaxShape::String, "WiFi network name", Some('s'))
            .named("password", SyntaxShape::String, "WiFi password", Some('p'))
            .input_output_type(Type::Nothing, Type::Any)
    }

    fn description(&self) -> &str {
        "Connect to a WiFi network, using a saved profile when available."
    }

    fn run(
        &self,
        _plugin: &EvoWifiPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let ssid = required_string_flag(call, "ssid", "ssid")?;
        let password = optional_string_flag(call, "password")?;

        let selection_provider = NushellNetworkSelectionInputProvider::new(Some(ssid));
        let status_output_provider = NushellStatusOutputProvider::default();
        let message_output_provider = NushellMessageOutputProvider::default();

        if let Some(password) = password {
            let password_provider = NushellPasswordInputProvider::new(Some(password));
            let command = WifiNewNetworkConnectCommand::new(
                selection_provider,
                password_provider,
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiNewNetworkConnectProvider,
                &status_output_provider,
                &message_output_provider,
            );

            command.execute().ok_or_else(|| {
                LabeledError::new("Could not connect to WiFi network")
                    .with_label("new network connect returned no value", call.head)
            })?;
        } else {
            let command = WifiNetworkConnectCommand::new(
                selection_provider,
                LinuxWpaWifiInterfaceProvider,
                LinuxWpaWifiStatusProvider,
                LinuxWpaWifiSavedNetworkProvider,
                LinuxWpaWifiConnectProvider,
                &status_output_provider,
                &message_output_provider,
            );

            command.execute().ok_or_else(|| {
                LabeledError::new("Could not connect to WiFi network")
                    .with_label("saved network connect returned no value", call.head)
            })?;
        }

        return_output_value(call.head, &status_output_provider, &message_output_provider)
    }
}

fn required_string_flag(
    call: &EvaluatedCall,
    name: &str,
    label: &str,
) -> Result<String, LabeledError> {
    call.get_flag::<String>(name)
        .map_err(|err| LabeledError::from(err).with_label(format!("invalid {label}"), call.head))?
        .ok_or_else(|| {
            LabeledError::new(format!("Missing required flag --{name}"))
                .with_label(format!("provide --{name}"), call.head)
        })
}

fn optional_string_flag(call: &EvaluatedCall, name: &str) -> Result<Option<String>, LabeledError> {
    call.get_flag::<String>(name)
        .map_err(|err| LabeledError::from(err).with_label(format!("invalid --{name}"), call.head))
}
