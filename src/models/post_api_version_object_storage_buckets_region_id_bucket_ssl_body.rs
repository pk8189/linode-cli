#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionObjectStorageBucketsRegionIdBucketSslBody {
    #[cfg_attr(feature = "cli", arg(id = "certificate", long = "certificate"))]
    pub certificate: String,
    #[cfg_attr(feature = "cli", arg(id = "private-key", long = "private-key"))]
    pub private_key: String,
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
