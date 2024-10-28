#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionObjectStorageBucketsBodyAclEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "authenticated-read"))]
    #[serde(rename = "authenticated-read")]
    AuthenticatedRead,
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
impl std::fmt::Display for PostApiVersionObjectStorageBucketsBodyAclEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionObjectStorageBucketsBodyAclEnum::AuthenticatedRead => {
                "authenticated-read"
            }
            PostApiVersionObjectStorageBucketsBodyAclEnum::Private => "private",
            PostApiVersionObjectStorageBucketsBodyAclEnum::PublicRead => "public-read",
            PostApiVersionObjectStorageBucketsBodyAclEnum::PublicReadWrite => {
                "public-read-write"
            }
        };
        write!(f, "{}", str_val)
    }
}
