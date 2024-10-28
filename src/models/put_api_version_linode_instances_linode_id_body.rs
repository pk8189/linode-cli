#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLinodeInstancesLinodeIdBody {
    #[cfg_attr(feature = "cli", arg(id = "alerts", long = "alerts"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdBodyAlerts>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<crate::models::PutApiVersionLinodeInstancesLinodeIdBodyAlerts>,
    #[cfg_attr(feature = "cli", arg(id = "backups", long = "backups"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdBodyBackups>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<crate::models::PutApiVersionLinodeInstancesLinodeIdBodyBackups>,
    #[cfg_attr(feature = "cli", arg(id = "capabilities", long = "capabilities"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "disk-encryption", long = "disk-encryption"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub disk_encryption: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "group", long = "group"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "has-user-data", long = "has-user-data"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_user_data: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "host-uuid", long = "host-uuid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_uuid: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "hypervisor", long = "hypervisor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdBodyHypervisorEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "image", long = "image"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "ipv4", long = "ipv4"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
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
    #[cfg_attr(feature = "cli", arg(id = "lke-cluster-id", long = "lke-cluster-id"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub lke_cluster_id: crate::core::patch::Patch<i64>,
    #[cfg_attr(feature = "cli", arg(id = "placement-group", long = "placement-group"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroup>
        )
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<crate::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroup>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub placement_group: crate::core::patch::Patch<
        crate::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroup,
    >,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "specs", long = "specs"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdBodySpecs>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specs: Option<crate::models::PutApiVersionLinodeInstancesLinodeIdBodySpecs>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdBodyStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "watchdog-enabled", long = "watchdog-enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog_enabled: Option<bool>,
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
