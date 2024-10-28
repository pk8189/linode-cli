#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLkeClustersBodyNodePoolsItem {
    #[cfg_attr(feature = "cli", arg(id = "autoscaler", long = "autoscaler"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLkeClustersBodyNodePoolsItemAutoscaler>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscaler: Option<
        crate::models::PostApiVersionLkeClustersBodyNodePoolsItemAutoscaler,
    >,
    #[cfg_attr(feature = "cli", arg(id = "count", long = "count"))]
    pub count: i64,
    #[cfg_attr(feature = "cli", arg(id = "disks", long = "disks"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLkeClustersBodyNodePoolsItemDisksItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<
        Vec<crate::models::PostApiVersionLkeClustersBodyNodePoolsItemDisksItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "labels", long = "labels"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLkeClustersBodyNodePoolsItemLabels>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::models::PostApiVersionLkeClustersBodyNodePoolsItemLabels>,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "taints", long = "taints"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<
        Vec<crate::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    pub type_field: String,
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
