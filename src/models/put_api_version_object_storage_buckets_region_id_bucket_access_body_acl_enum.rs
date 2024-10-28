#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum {
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
for PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::AuthenticatedRead => {
                "authenticated-read"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::Custom => {
                "custom"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::Private => {
                "private"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::PublicRead => {
                "public-read"
            }
            PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::PublicReadWrite => {
                "public-read-write"
            }
        };
        write!(f, "{}", str_val)
    }
}
