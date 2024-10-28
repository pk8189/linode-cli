#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLkeClustersClusterIdBodyControlPlane {
    #[cfg_attr(feature = "cli", arg(id = "acl", long = "acl"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLkeClustersClusterIdBodyControlPlaneAcl>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<crate::models::PutApiVersionLkeClustersClusterIdBodyControlPlaneAcl>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "high-availability", long = "high-availability")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<bool>,
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
