#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionDatabasesTypesTypeIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "class", long = "class"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "deprecated", long = "deprecated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "disk", long = "disk"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "engines", long = "engines"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionDatabasesTypesTypeIdResponseEngines>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<crate::models::GetApiVersionDatabasesTypesTypeIdResponseEngines>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "memory", long = "memory"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "vcpus", long = "vcpus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
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
