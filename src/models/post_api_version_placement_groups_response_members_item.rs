#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionPlacementGroupsResponseMembersItem {
    #[cfg_attr(feature = "cli", arg(id = "is-compliant", long = "is-compliant"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i64>,
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
