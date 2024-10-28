#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
    #[cfg_attr(feature = "cli", arg(id = "bucket", long = "bucket"))]
    pub bucket: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessBody,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
    #[cfg_attr(feature = "cli", arg(id = "bucket", long = "bucket"))]
    pub bucket: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessBody,
}
