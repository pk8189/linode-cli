#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionAccountSettingsBody {
    #[cfg_attr(feature = "cli", arg(id = "backups-enabled", long = "backups-enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups_enabled: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "longview-subscription", long = "longview-subscription")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longview_subscription: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "managed", long = "managed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "network-helper", long = "network-helper"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_helper: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "object-storage", long = "object-storage"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_storage: Option<
        crate::models::PutApiVersionAccountSettingsBodyObjectStorageEnum,
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
