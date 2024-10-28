#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionAccountPaymentsResponseWarningsItem {
    #[cfg_attr(feature = "cli", arg(id = "details", long = "details"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "title", long = "title"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
