#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLongviewClientsClientIdResponseApps {
    #[cfg_attr(feature = "cli", arg(id = "apache", long = "apache"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apache: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "mysql", long = "mysql"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "nginx", long = "nginx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nginx: Option<bool>,
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
