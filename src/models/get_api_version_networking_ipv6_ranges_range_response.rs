#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionNetworkingIpv6RangesRangeResponse {
    #[cfg_attr(feature = "cli", arg(id = "is-bgp", long = "is-bgp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bgp: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "linodes", long = "linodes"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linodes: Option<Vec<i64>>,
    #[cfg_attr(feature = "cli", arg(id = "prefix", long = "prefix"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "range", long = "range"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
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
