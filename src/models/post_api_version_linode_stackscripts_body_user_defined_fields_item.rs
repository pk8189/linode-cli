#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeStackscriptsBodyUserDefinedFieldsItem {
    #[cfg_attr(feature = "cli", arg(id = "default", long = "default"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "example", long = "example"))]
    pub example: String,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    pub label: String,
    #[cfg_attr(feature = "cli", arg(id = "many-of", long = "many-of"))]
    #[serde(rename = "manyOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub many_of: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    pub name: String,
    #[cfg_attr(feature = "cli", arg(id = "one-of", long = "one-of"))]
    #[serde(rename = "oneOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(
        feature = "cli",
        arg(
            id = "additional-props",
            long = "additional-props",
            value_parser = crate::core::clap::parse_json::<std::collections::HashMap<String,
            serde_json::Value>>,
            default_value = "{}",
        )
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
