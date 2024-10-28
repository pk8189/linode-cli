#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "authenticated-read"))]
    #[serde(rename = "authenticated-read")]
    AuthenticatedRead,
    #[cfg_attr(feature = "cli", value(name = "custom"))]
    #[serde(rename = "custom")]
    Custom,
    #[cfg_attr(feature = "cli", value(name = "private"))]
    #[serde(rename = "private")]
    Private,
    #[cfg_attr(feature = "cli", value(name = "public-read"))]
    #[serde(rename = "public-read")]
    PublicRead,
    #[cfg_attr(feature = "cli", value(name = "public-read-write"))]
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
}
impl std::fmt::Display
for PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum::AuthenticatedRead => {
                "authenticated-read"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum::Custom => {
                "custom"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum::Private => {
                "private"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum::PublicRead => {
                "public-read"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum::PublicReadWrite => {
                "public-read-write"
            }
        };
        write!(f, "{}", str_val)
    }
}
