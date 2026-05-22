use evo_wifi_provider_nushell::values::{NushellRecord, NushellValue};
use nu_protocol::{Record, Span, Value};

pub fn nushell_value_to_value(value: &NushellValue, span: Span) -> Value {
    match value {
        NushellValue::String(value) => Value::string(value.to_owned(), span),
        NushellValue::Int(value) => Value::int(*value, span),
        NushellValue::Record(value) => record_to_value(value, span),
        NushellValue::List(values) => Value::list(
            values
                .iter()
                .map(|value| nushell_value_to_value(value, span))
                .collect(),
            span,
        ),
    }
}

fn record_to_value(record: &NushellRecord, span: Span) -> Value {
    let mut nu_record = Record::new();

    for field in record.fields() {
        nu_record.push(
            field.name.clone(),
            nushell_value_to_value(&field.value, span),
        );
    }

    Value::record(nu_record, span)
}

#[cfg(test)]
mod tests {
    use super::*;
    use evo_wifi_provider_nushell::values::NushellRecord;

    #[test]
    fn converts_record_to_nu_value() {
        let span = Span::test_data();
        let value = NushellRecord::from([
            ("ssid", NushellValue::string("evo-wifi")),
            ("state", NushellValue::string("COMPLETED")),
        ]);

        let value = nushell_value_to_value(&value.into(), span);

        assert!(matches!(value, Value::Record { .. }));
    }

    #[test]
    fn converts_list_to_nu_value() {
        let span = Span::test_data();
        let value = NushellValue::list(vec![
            NushellRecord::from([("ssid", NushellValue::string("evo-wifi"))]).into(),
        ]);

        let value = nushell_value_to_value(&value, span);

        assert!(matches!(value, Value::List { .. }));
    }
}
