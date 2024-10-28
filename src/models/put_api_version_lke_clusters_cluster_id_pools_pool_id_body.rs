#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLkeClustersClusterIdPoolsPoolIdBody {
    #[cfg_attr(feature = "cli", arg(id = "autoscaler", long = "autoscaler"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyAutoscaler>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscaler: Option<
        crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyAutoscaler,
    >,
    #[cfg_attr(feature = "cli", arg(id = "count", long = "count"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "labels", long = "labels"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyLabels>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<
        crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyLabels,
    >,
    #[cfg_attr(feature = "cli", arg(id = "taints", long = "taints"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<
        Vec<crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItem>,
    >,
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
