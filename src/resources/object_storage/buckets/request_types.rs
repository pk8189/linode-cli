#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionObjectStorageBucketsRegionIdBucketApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
    #[cfg_attr(feature = "cli", arg(id = "bucket", long = "bucket"))]
    pub bucket: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionObjectStorageBucketsApiVersionEnum,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetByRegionIdRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionObjectStorageBucketsRegionIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetByBucketRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
    #[cfg_attr(feature = "cli", arg(id = "bucket", long = "bucket"))]
    pub bucket: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionObjectStorageBucketsApiVersionEnum,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionObjectStorageBucketsBody,
}
