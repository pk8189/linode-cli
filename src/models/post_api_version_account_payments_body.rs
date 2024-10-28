#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionAccountPaymentsBody {
    #[cfg_attr(
        feature = "cli",
        arg(id = "payment-method-id", long = "payment-method-id")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "usd", long = "usd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<String>,
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
