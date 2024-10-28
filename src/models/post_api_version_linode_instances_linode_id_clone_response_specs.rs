#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdCloneResponseSpecs {
    #[cfg_attr(feature = "cli", arg(id = "disk", long = "disk"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "gpus", long = "gpus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpus: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "memory", long = "memory"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "transfer", long = "transfer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "vcpus", long = "vcpus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
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
