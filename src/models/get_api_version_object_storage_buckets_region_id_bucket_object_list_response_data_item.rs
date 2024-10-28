#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionObjectStorageBucketsRegionIdBucketObjectListResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "etag", long = "etag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "last-modified", long = "last-modified"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "owner", long = "owner"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "size", long = "size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
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
