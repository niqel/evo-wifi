use crate::EvoWifiPlugin;
use crate::values::nushell_value_to_value;
use evo_wifi_core::commands::WifiAvailableNetworksShowCommand;
use evo_wifi_provider_linux_wpa::inputs::linux_wpa::{
    LinuxWpaWifiInterfaceProvider, LinuxWpaWifiScanProvider,
};
use evo_wifi_provider_nushell::outputs::nushell::NushellNetworkOutputProvider;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, Type, Value};

pub struct EvoWifiNetworksCommand;

impl SimplePluginCommand for EvoWifiNetworksCommand {
    type Plugin = EvoWifiPlugin;

    fn name(&self) -> &str {
        "evo-nu-wifi networks"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::List(Box::new(Type::Any)))
    }

    fn description(&self) -> &str {
        "Return scanned WiFi networks as a Nushell table."
    }

    fn run(
        &self,
        _plugin: &EvoWifiPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let output_provider = NushellNetworkOutputProvider::default();
        let command = WifiAvailableNetworksShowCommand::new(
            LinuxWpaWifiInterfaceProvider,
            LinuxWpaWifiScanProvider,
            &output_provider,
        );

        command.execute().ok_or_else(|| {
            LabeledError::new("Could not scan WiFi networks")
                .with_label("scan provider returned no value", call.head)
        })?;

        output_provider
            .value()
            .map(|value| nushell_value_to_value(value, call.head))
            .ok_or_else(|| {
                LabeledError::new("Could not return WiFi networks")
                    .with_label("Nushell output provider produced no value", call.head)
            })
    }
}
