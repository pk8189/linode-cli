#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionImagesBody {
    #[cfg_attr(feature = "cli", arg(id = "cloud-init", long = "cloud-init"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_init: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "disk-id", long = "disk-id"))]
    pub disk_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
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
