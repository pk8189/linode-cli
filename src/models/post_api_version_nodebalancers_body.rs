#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionNodebalancersBody {
    #[cfg_attr(
        feature = "cli",
        arg(id = "client-conn-throttle", long = "client-conn-throttle")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_conn_throttle: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "configs", long = "configs"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionNodebalancersBodyConfigsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<crate::models::PostApiVersionNodebalancersBodyConfigsItem>>,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    pub region: String,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
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
