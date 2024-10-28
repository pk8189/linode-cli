#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionObjectStorageKeysBodyBucketAccessItem {
    #[cfg_attr(feature = "cli", arg(id = "bucket-name", long = "bucket-name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "permissions", long = "permissions"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        crate::models::PostApiVersionObjectStorageKeysBodyBucketAccessItemPermissionsEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
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
