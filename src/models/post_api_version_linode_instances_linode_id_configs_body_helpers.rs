#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdConfigsBodyHelpers {
    #[cfg_attr(
        feature = "cli",
        arg(id = "devtmpfs-automount", long = "devtmpfs-automount")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devtmpfs_automount: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "distro", long = "distro"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distro: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "modules-dep", long = "modules-dep"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules_dep: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "network", long = "network"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "updatedb-disabled", long = "updatedb-disabled")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updatedb_disabled: Option<bool>,
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
