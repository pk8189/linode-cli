#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountLoginsResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "datetime", long = "datetime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "ip", long = "ip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "restricted", long = "restricted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::GetApiVersionAccountLoginsResponseDataItemStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "username", long = "username"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
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
