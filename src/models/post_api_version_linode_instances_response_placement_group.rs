#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesResponsePlacementGroup {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "placement-group-policy", long = "placement-group-policy")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_policy: Option<
        crate::models::PostApiVersionLinodeInstancesResponsePlacementGroupPlacementGroupPolicyEnum,
    >,
    #[cfg_attr(
        feature = "cli",
        arg(id = "placement-group-type", long = "placement-group-type")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_type: Option<
        crate::models::PostApiVersionLinodeInstancesResponsePlacementGroupPlacementGroupTypeEnum,
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
