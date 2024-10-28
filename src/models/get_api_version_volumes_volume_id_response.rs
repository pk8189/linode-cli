#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionVolumesVolumeIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "encryption", long = "encryption"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<
        crate::models::GetApiVersionVolumesVolumeIdResponseEncryptionEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "filesystem-path", long = "filesystem-path"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_path: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "hardware-type", long = "hardware-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_type: Option<
        crate::models::GetApiVersionVolumesVolumeIdResponseHardwareTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub linode_id: crate::core::patch::Patch<i64>,
    #[cfg_attr(feature = "cli", arg(id = "linode-label", long = "linode-label"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub linode_label: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "size", long = "size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::GetApiVersionVolumesVolumeIdResponseStatusEnum>,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
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
