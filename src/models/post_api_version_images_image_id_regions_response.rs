#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionImagesImageIdRegionsResponse {
    #[cfg_attr(feature = "cli", arg(id = "capabilities", long = "capabilities"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "created-by", long = "created-by"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "deprecated", long = "deprecated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub description: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "eol", long = "eol"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub eol: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "expiry", long = "expiry"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub expiry: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "is-public", long = "is-public"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "regions", long = "regions"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionImagesImageIdRegionsResponseRegionsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<
        Vec<crate::models::PostApiVersionImagesImageIdRegionsResponseRegionsItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "size", long = "size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::PostApiVersionImagesImageIdRegionsResponseStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "total-size", long = "total-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<
        crate::models::PostApiVersionImagesImageIdRegionsResponseTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "vendor", long = "vendor"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub vendor: crate::core::patch::Patch<String>,
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
