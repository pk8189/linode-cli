#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeTypesResponseDataItemAddonsBackupsRegionPricesItem {
    #[cfg_attr(feature = "cli", arg(id = "hourly", long = "hourly"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "monthly", long = "monthly"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly: Option<f64>,
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
