use crate::values::nushell_value_to_value;
use evo_wifi_provider_nushell::outputs::nushell::{
    NushellMessageOutputProvider, NushellStatusOutputProvider,
};
use nu_protocol::{LabeledError, Span, Value};

pub fn return_output_value(
    head: Span,
    status_provider: &NushellStatusOutputProvider,
    message_provider: &NushellMessageOutputProvider,
) -> Result<Value, LabeledError> {
    if let Some(value) = status_provider.value() {
        return Ok(nushell_value_to_value(value, head));
    }

    if let Some(value) = message_provider.value() {
        return Ok(nushell_value_to_value(value, head));
    }

    Err(LabeledError::new("Could not return WiFi result")
        .with_label("No output provider produced a value", head))
}

pub fn return_message_value(
    head: Span,
    message_provider: &NushellMessageOutputProvider,
) -> Result<Value, LabeledError> {
    message_provider
        .value()
        .map(|value| nushell_value_to_value(value, head))
        .ok_or_else(|| {
            LabeledError::new("Could not return WiFi message")
                .with_label("Nushell message provider produced no value", head)
        })
}
