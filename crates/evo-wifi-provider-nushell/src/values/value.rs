#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NushellValue {
    String(String),
    Int(i64),
    Record(NushellRecord),
    List(Vec<NushellValue>),
}

impl NushellValue {
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    pub fn int(value: i64) -> Self {
        Self::Int(value)
    }

    pub fn list(values: Vec<NushellValue>) -> Self {
        Self::List(values)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NushellRecord {
    fields: Vec<NushellRecordField>,
}

impl NushellRecord {
    pub fn fields(&self) -> &[NushellRecordField] {
        &self.fields
    }
}

impl<const N: usize> From<[(&str, NushellValue); N]> for NushellRecord {
    fn from(fields: [(&str, NushellValue); N]) -> Self {
        Self {
            fields: fields
                .into_iter()
                .map(|(name, value)| NushellRecordField {
                    name: name.to_owned(),
                    value,
                })
                .collect(),
        }
    }
}

impl From<NushellRecord> for NushellValue {
    fn from(value: NushellRecord) -> Self {
        Self::Record(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NushellRecordField {
    pub name: String,
    pub value: NushellValue,
}
