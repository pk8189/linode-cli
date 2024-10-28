#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountTransferResponseRegionTransfersItem {
    #[cfg_attr(feature = "cli", arg(id = "billable", long = "billable"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "quota", long = "quota"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "used", long = "used"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
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
