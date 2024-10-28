#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAcl {
    #[cfg_attr(feature = "cli", arg(id = "addresses", long = "addresses"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAclAddresses>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<
        crate::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAclAddresses,
    >,
    #[cfg_attr(feature = "cli", arg(id = "enabled", long = "enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "revision-id", long = "revision-id"))]
    #[serde(rename = "revision-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
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
