#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionRegionsRegionIdResponseResolvers {
    #[cfg_attr(feature = "cli", arg(id = "ipv4", long = "ipv4"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "ipv6", long = "ipv6"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
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
