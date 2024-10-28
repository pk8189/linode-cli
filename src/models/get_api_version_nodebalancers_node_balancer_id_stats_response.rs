#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionNodebalancersNodeBalancerIdStatsResponse {
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionNodebalancersNodeBalancerIdStatsResponseData>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<
        crate::models::GetApiVersionNodebalancersNodeBalancerIdStatsResponseData,
    >,
    #[cfg_attr(feature = "cli", arg(id = "title", long = "title"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
