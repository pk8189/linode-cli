#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionNodebalancersNodeBalancerIdConfigsResponseDataItemNodesStatus {
    #[cfg_attr(feature = "cli", arg(id = "down", long = "down"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "up", long = "up"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<i64>,
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
