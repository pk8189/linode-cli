#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectAclApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
    #[cfg_attr(feature = "cli", arg(id = "bucket", long = "bucket"))]
    pub bucket: String,
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    pub name: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "region-id", long = "region-id"))]
    pub region_id: String,
    #[cfg_attr(feature = "cli", arg(id = "bucket", long = "bucket"))]
    pub bucket: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBody,
}
