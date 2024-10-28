#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionObjectStorageKeysResponseBucketAccessItemPermissionsEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "read_only"))]
    #[serde(rename = "read_only")]
    ReadOnly,
    #[cfg_attr(feature = "cli", value(name = "read_write"))]
    #[serde(rename = "read_write")]
    ReadWrite,
}
impl std::fmt::Display
for PostApiVersionObjectStorageKeysResponseBucketAccessItemPermissionsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionObjectStorageKeysResponseBucketAccessItemPermissionsEnum::ReadOnly => {
                "read_only"
            }
            PostApiVersionObjectStorageKeysResponseBucketAccessItemPermissionsEnum::ReadWrite => {
                "read_write"
            }
        };
        write!(f, "{}", str_val)
    }
}
