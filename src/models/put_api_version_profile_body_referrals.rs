#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionProfileBodyReferrals {
    #[cfg_attr(feature = "cli", arg(id = "code", long = "code"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "completed", long = "completed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "credit", long = "credit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "pending", long = "pending"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "total", long = "total"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "url", long = "url"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
