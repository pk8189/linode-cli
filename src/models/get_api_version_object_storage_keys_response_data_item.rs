#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionObjectStorageKeysResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "access-key", long = "access-key"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "bucket-access", long = "bucket-access"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionObjectStorageKeysResponseDataItemBucketAccessItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_access: Option<
        Vec<
            crate::models::GetApiVersionObjectStorageKeysResponseDataItemBucketAccessItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "limited", long = "limited"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "regions", long = "regions"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionObjectStorageKeysResponseDataItemRegionsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<
        Vec<crate::models::GetApiVersionObjectStorageKeysResponseDataItemRegionsItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "secret-key", long = "secret-key"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
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
