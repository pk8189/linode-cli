#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountNotificationsResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "body", long = "body"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub body: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "entity", long = "entity"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionAccountNotificationsResponseDataItemEntity>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<
        crate::models::GetApiVersionAccountNotificationsResponseDataItemEntity,
    >,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "message", long = "message"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "severity", long = "severity"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<
        crate::models::GetApiVersionAccountNotificationsResponseDataItemSeverityEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<
        crate::models::GetApiVersionAccountNotificationsResponseDataItemTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "until", long = "until"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "when", long = "when"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
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
