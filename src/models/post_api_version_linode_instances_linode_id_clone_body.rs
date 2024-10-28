#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdCloneBody {
    #[cfg_attr(feature = "cli", arg(id = "backups-enabled", long = "backups-enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups_enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "configs", long = "configs"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<i64>>,
    #[cfg_attr(feature = "cli", arg(id = "disks", long = "disks"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<i64>>,
    #[cfg_attr(feature = "cli", arg(id = "group", long = "group"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "metadata", long = "metadata"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyMetadata>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyMetadata,
    >,
    #[cfg_attr(feature = "cli", arg(id = "placement-group", long = "placement-group"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyPlacementGroup>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyPlacementGroup,
    >,
    #[cfg_attr(feature = "cli", arg(id = "private-ip", long = "private-ip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
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
