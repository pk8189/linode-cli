#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionVolumesVolumeIdAttachBody {
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(
        feature = "cli",
        arg(id = "persist-across-boots", long = "persist-across-boots")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_across_boots: Option<bool>,
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
