#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLkeClustersClusterIdControlPlaneAclResponse400 {
    #[cfg_attr(feature = "cli", arg(id = "errors", long = "errors"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLkeClustersClusterIdControlPlaneAclResponse400ErrorsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<
        Vec<
            crate::models::GetApiVersionLkeClustersClusterIdControlPlaneAclResponse400ErrorsItem,
        >,
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
