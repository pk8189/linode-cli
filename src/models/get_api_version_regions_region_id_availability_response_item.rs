#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionRegionsRegionIdAvailabilityResponseItem {
    #[cfg_attr(feature = "cli", arg(id = "available", long = "available"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "plan", long = "plan"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
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
