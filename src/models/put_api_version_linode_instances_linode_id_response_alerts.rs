#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLinodeInstancesLinodeIdResponseAlerts {
    #[cfg_attr(feature = "cli", arg(id = "cpu", long = "cpu"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "io", long = "io"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "network-in", long = "network-in"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_in: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "network-out", long = "network-out"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_out: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "transfer-quota", long = "transfer-quota"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_quota: Option<i64>,
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
