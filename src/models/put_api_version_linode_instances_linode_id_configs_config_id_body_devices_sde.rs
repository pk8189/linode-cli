#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSde {
    #[cfg_attr(feature = "cli", arg(id = "disk-id", long = "disk-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "volume-id", long = "volume-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<i64>,
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
