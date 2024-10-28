#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectListApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
    #[cfg_attr(feature = "cli", arg(id = "bucket", long = "bucket"))]
    pub bucket: String,
    #[cfg_attr(feature = "cli", arg(id = "delimiter", long = "delimiter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "marker", long = "marker"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "page-size", long = "page-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "prefix", long = "prefix"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}
