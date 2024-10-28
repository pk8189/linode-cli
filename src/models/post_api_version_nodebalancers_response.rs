#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionNodebalancersResponse {
    #[cfg_attr(
        feature = "cli",
        arg(id = "client-conn-throttle", long = "client-conn-throttle")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_conn_throttle: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "hostname", long = "hostname"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "ipv4", long = "ipv4"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "ipv6", long = "ipv6"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub ipv6: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "transfer", long = "transfer"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionNodebalancersResponseTransfer>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<crate::models::PostApiVersionNodebalancersResponseTransfer>,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
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
