use crate::draft::HasSchemaUri;
use serde::Deserialize;
use serde_json::Value;
use std::clone::Clone;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "$id")]
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub r#type: SchemaType,
    pub properties: Value,
    pub additional_properties: Option<bool>,
}

impl HasSchemaUri for Schema {
    fn schema_uri(&self) -> &str {
        self.schema.as_str()
    }
}
