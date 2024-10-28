#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLkeClustersClusterIdPoolsPoolIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "autoscaler", long = "autoscaler"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseAutoscaler>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscaler: Option<
        crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseAutoscaler,
    >,
    #[cfg_attr(feature = "cli", arg(id = "count", long = "count"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "disk-encryption", long = "disk-encryption"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption: Option<
        crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseDiskEncryptionEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "disks", long = "disks"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<
        Vec<crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "labels", long = "labels"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseLabels>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<
        crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseLabels,
    >,
    #[cfg_attr(feature = "cli", arg(id = "nodes", long = "nodes"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseNodesItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<
        Vec<crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseNodesItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "taints", long = "taints"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseTaintsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<
        Vec<
            crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseTaintsItem,
        >,
    >,
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
