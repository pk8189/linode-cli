#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionNodebalancersNodeBalancerIdStatsResponseDataTraffic {
    #[cfg_attr(feature = "cli", arg(id = "in-field", long = "in-field"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_field: Option<Vec<f64>>,
    #[cfg_attr(feature = "cli", arg(id = "out", long = "out"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out: Option<Vec<f64>>,
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
