#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionSupportTicketsResponse {
    #[cfg_attr(feature = "cli", arg(id = "attachments", long = "attachments"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "closable", long = "closable"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closable: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "closed", long = "closed"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub closed: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "entity", long = "entity"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionSupportTicketsResponseEntity>
        )
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<crate::models::PostApiVersionSupportTicketsResponseEntity>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub entity: crate::core::patch::Patch<
        crate::models::PostApiVersionSupportTicketsResponseEntity,
    >,
    #[cfg_attr(feature = "cli", arg(id = "gravatar-id", long = "gravatar-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "opened", long = "opened"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "opened-by", long = "opened-by"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::PostApiVersionSupportTicketsResponseStatusEnum>,
    #[cfg_attr(feature = "cli", arg(id = "summary", long = "summary"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "updated-by", long = "updated-by"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub updated_by: crate::core::patch::Patch<String>,
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
