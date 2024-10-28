#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBody {
    #[cfg_attr(feature = "cli", arg(id = "acl", long = "acl"))]
    pub acl: crate::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum,
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    pub name: String,
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
