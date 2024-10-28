#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLkeClustersBody {
    #[cfg_attr(feature = "cli", arg(id = "control-plane", long = "control-plane"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLkeClustersBodyControlPlane>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane: Option<crate::models::PostApiVersionLkeClustersBodyControlPlane>,
    #[cfg_attr(feature = "cli", arg(id = "k8s-version", long = "k8s-version"))]
    pub k8s_version: String,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    pub label: String,
    #[cfg_attr(feature = "cli", arg(id = "node-pools", long = "node-pools"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLkeClustersBodyNodePoolsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub node_pools: Vec<crate::models::PostApiVersionLkeClustersBodyNodePoolsItem>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    pub region: String,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
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
