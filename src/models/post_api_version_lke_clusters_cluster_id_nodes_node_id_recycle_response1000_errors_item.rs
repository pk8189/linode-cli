#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLkeClustersClusterIdNodesNodeIdRecycleResponse1000ErrorsItem {
    #[cfg_attr(feature = "cli", arg(id = "field", long = "field"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "reason", long = "reason"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
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
