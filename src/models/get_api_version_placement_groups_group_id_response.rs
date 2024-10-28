#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionPlacementGroupsGroupIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "is-compliant", long = "is-compliant"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "members", long = "members"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionPlacementGroupsGroupIdResponseMembersItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<
        Vec<crate::models::GetApiVersionPlacementGroupsGroupIdResponseMembersItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "migrations", long = "migrations"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionPlacementGroupsGroupIdResponseMigrations>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrations: Option<
        crate::models::GetApiVersionPlacementGroupsGroupIdResponseMigrations,
    >,
    #[cfg_attr(
        feature = "cli",
        arg(id = "placement-group-policy", long = "placement-group-policy")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_policy: Option<
        crate::models::GetApiVersionPlacementGroupsGroupIdResponsePlacementGroupPolicyEnum,
    >,
    #[cfg_attr(
        feature = "cli",
        arg(id = "placement-group-type", long = "placement-group-type")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_type: Option<
        crate::models::GetApiVersionPlacementGroupsGroupIdResponsePlacementGroupTypeEnum,
    >,
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
