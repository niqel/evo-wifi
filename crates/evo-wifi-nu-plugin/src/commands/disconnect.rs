use crate::EvoWifiPlugin;
use crate::commands::common::return_output_value;
use evo_wifi_core::commands::WifiConnectionDisconnectCommand;
use evo_wifi_provider_linux_wpa::actions::linux_wpa::LinuxWpaWifiDisconnectProvider;
use evo_wifi_provider_linux_wpa::inputs::linux_wpa::{
    LinuxWpaWifiInterfaceProvider, LinuxWpaWifiStatusProvider,
};
use evo_wifi_provider_nushell::outputs::nushell::{
    NushellMessageOutputProvider, NushellStatusOutputProvider,
};
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, Type, Value};

pub struct EvoWifiDisconnectCommand;

impl SimplePluginCommand for EvoWifiDisconnectCommand {
    type Plugin = EvoWifiPlugin;

    fn name(&self) -> &str {
        "evo-nu-wifi disconnect"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).input_output_type(Type::Nothing, Type::Any)
    }

    fn description(&self) -> &str {
        "Disconnect from the current WiFi network."
    }

    fn run(
        &self,
        _plugin: &EvoWifiPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let status_output_provider = NushellStatusOutputProvider::default();
        let message_output_provider = NushellMessageOutputProvider::default();
        let command = WifiConnectionDisconnectCommand::new(
            LinuxWpaWifiInterfaceProvider,
            LinuxWpaWifiStatusProvider,
            LinuxWpaWifiDisconnectProvider,
            &status_output_provider,
            &message_output_provider,
        );

        command.execute().ok_or_else(|| {
            LabeledError::new("Could not disconnect from WiFi")
                .with_label("disconnect returned no value", call.head)
        })?;

        return_output_value(call.head, &status_output_provider, &message_output_provider)
    }
}
