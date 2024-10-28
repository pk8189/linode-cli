#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountEventsResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "action", long = "action"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<
        crate::models::GetApiVersionAccountEventsResponseDataItemActionEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "duration", long = "duration"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "entity", long = "entity"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionAccountEventsResponseDataItemEntity>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<crate::models::GetApiVersionAccountEventsResponseDataItemEntity>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "message", long = "message"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub message: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "percent-complete", long = "percent-complete"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "rate", long = "rate"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "read", long = "read"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "secondary-entity", long = "secondary-entity"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionAccountEventsResponseDataItemSecondaryEntity>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_entity: Option<
        crate::models::GetApiVersionAccountEventsResponseDataItemSecondaryEntity,
    >,
    #[cfg_attr(feature = "cli", arg(id = "seen", long = "seen"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::GetApiVersionAccountEventsResponseDataItemStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "time-remaining", long = "time-remaining"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub time_remaining: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "username", long = "username"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub username: crate::core::patch::Patch<String>,
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
