#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionTagsTagLabelResponseDataItemDataObj0Backups {
    #[cfg_attr(feature = "cli", arg(id = "available", long = "available"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "enabled", long = "enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "last-successful", long = "last-successful"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "schedule", long = "schedule"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionTagsTagLabelResponseDataItemDataObj0BackupsSchedule>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<
        crate::models::GetApiVersionTagsTagLabelResponseDataItemDataObj0BackupsSchedule,
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
