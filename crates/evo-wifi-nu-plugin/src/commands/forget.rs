use crate::EvoWifiPlugin;
use crate::commands::common::return_message_value;
use evo_wifi_core::commands::WifiSavedNetworkForgetCommand;
use evo_wifi_provider_linux_wpa::actions::linux_wpa::LinuxWpaWifiForgetProvider;
use evo_wifi_provider_linux_wpa::inputs::linux_wpa::{
    LinuxWpaWifiInterfaceProvider, LinuxWpaWifiSavedNetworkProvider,
};
use evo_wifi_provider_nushell::inputs::nushell::NushellNetworkSelectionInputProvider;
use evo_wifi_provider_nushell::outputs::nushell::NushellMessageOutputProvider;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type, Value};

pub struct EvoWifiForgetCommand;

impl SimplePluginCommand for EvoWifiForgetCommand {
    type Plugin = EvoWifiPlugin;

    fn name(&self) -> &str {
        "evo-nu-wifi forget"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required_named("ssid", SyntaxShape::String, "WiFi network name", Some('s'))
            .input_output_type(Type::Nothing, Type::String)
    }

    fn description(&self) -> &str {
        "Forget a saved WiFi network."
    }

    fn run(
        &self,
        _plugin: &EvoWifiPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let ssid = call
            .get_flag::<String>("ssid")
            .map_err(|err| LabeledError::from(err).with_label("invalid --ssid", call.head))?
            .ok_or_else(|| {
                LabeledError::new("Missing required flag --ssid")
                    .with_label("provide --ssid", call.head)
            })?;

        let input_provider = NushellNetworkSelectionInputProvider::new(Some(ssid));
        let message_output_provider = NushellMessageOutputProvider::default();
        let command = WifiSavedNetworkForgetCommand::new(
            input_provider,
            LinuxWpaWifiInterfaceProvider,
            LinuxWpaWifiSavedNetworkProvider,
            LinuxWpaWifiForgetProvider,
            &message_output_provider,
        );

        command.execute().ok_or_else(|| {
            LabeledError::new("Could not forget WiFi network")
                .with_label("forget returned no value", call.head)
        })?;

        return_message_value(call.head, &message_output_provider)
    }
}
