#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "mode", long = "mode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<
        crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdResponseModeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "nodebalancer-id", long = "nodebalancer-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodebalancer_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdResponseStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "weight", long = "weight"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
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
