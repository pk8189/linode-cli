#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionObjectStorageBucketsRegionIdBucketObjectListResponse {
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectListResponseDataItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<
        Vec<
            crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectListResponseDataItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "is-truncated", long = "is-truncated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "next-marker", long = "next-marker"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub next_marker: crate::core::patch::Patch<String>,
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
