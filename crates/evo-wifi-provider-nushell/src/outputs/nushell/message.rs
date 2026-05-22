use crate::values::NushellValue;
use evo_wifi_core::contracts::WifiMessageOutputContract;
use std::sync::OnceLock;

#[derive(Debug, Default)]
pub struct NushellMessageOutputProvider {
    value: OnceLock<NushellValue>,
}

impl NushellMessageOutputProvider {
    pub fn value(&self) -> Option<&NushellValue> {
        self.value.get()
    }

    pub fn into_value(self) -> Option<NushellValue> {
        self.value.into_inner()
    }
}

impl WifiMessageOutputContract for NushellMessageOutputProvider {
    fn provide(&self, message: &str) {
        let _ = self.value.set(NushellValue::string(message));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_message_as_nushell_string() {
        let provider = NushellMessageOutputProvider::default();

        provider.provide("connected");

        assert_eq!(
            provider.into_value(),
            Some(NushellValue::string("connected"))
        );
    }
}
