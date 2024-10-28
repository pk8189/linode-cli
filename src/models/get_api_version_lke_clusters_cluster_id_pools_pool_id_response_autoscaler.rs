#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseAutoscaler {
    #[cfg_attr(feature = "cli", arg(id = "enabled", long = "enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "max", long = "max"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "min", long = "min"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
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
