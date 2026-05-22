use crate::EvoWifiPlugin;
use crate::values::nushell_value_to_value;
use evo_wifi_core::commands::WifiConnectionStatusShowCommand;
use evo_wifi_provider_linux_wpa::inputs::linux_wpa::{
    LinuxWpaWifiInterfaceProvider, LinuxWpaWifiStatusProvider,
};
use evo_wifi_provider_nushell::outputs::nushell::NushellStatusOutputProvider;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, Type, Value};

pub struct EvoWifiStatusCommand;

impl SimplePluginCommand for EvoWifiStatusCommand {
    type Plugin = EvoWifiPlugin;

    fn name(&self) -> &str {
        "evo-nu-wifi status"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).input_output_type(Type::Nothing, Type::Record(Box::new([])))
    }

    fn description(&self) -> &str {
        "Return the current WiFi connection status as a Nushell record."
    }

    fn run(
        &self,
        _plugin: &EvoWifiPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let output_provider = NushellStatusOutputProvider::default();
        let command = WifiConnectionStatusShowCommand::new(
            LinuxWpaWifiInterfaceProvider,
            LinuxWpaWifiStatusProvider,
            &output_provider,
        );

        command.execute().ok_or_else(|| {
            LabeledError::new("Could not resolve WiFi status")
                .with_label("status provider returned no value", call.head)
        })?;

        output_provider
            .value()
            .map(|value| nushell_value_to_value(value, call.head))
            .ok_or_else(|| {
                LabeledError::new("Could not return WiFi status")
                    .with_label("Nushell output provider produced no value", call.head)
            })
    }
}
