#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "available", long = "available"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "configs", long = "configs"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "disks", long = "disks"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseDisksItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<
        Vec<
            crate::models::GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseDisksItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "finished", long = "finished"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub label: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
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
