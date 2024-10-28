#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdMigrateBody {
    #[cfg_attr(feature = "cli", arg(id = "placement-group", long = "placement-group"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdMigrateBodyPlacementGroup>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdMigrateBodyPlacementGroup,
    >,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdMigrateBodyTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "upgrade", long = "upgrade"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade: Option<bool>,
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
