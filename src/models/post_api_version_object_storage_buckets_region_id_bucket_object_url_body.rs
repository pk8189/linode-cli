#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionObjectStorageBucketsRegionIdBucketObjectUrlBody {
    #[cfg_attr(feature = "cli", arg(id = "content-type", long = "content-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "expires-in", long = "expires-in"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "method", long = "method"))]
    pub method: String,
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    pub name: String,
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
