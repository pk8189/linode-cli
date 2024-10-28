#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLkeClustersClusterIdControlPlaneAclBody {
    #[cfg_attr(feature = "cli", arg(id = "acl", long = "acl"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAcl>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<
        crate::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAcl,
    >,
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
