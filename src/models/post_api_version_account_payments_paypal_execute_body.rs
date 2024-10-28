#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionAccountPaymentsPaypalExecuteBody {
    #[cfg_attr(feature = "cli", arg(id = "payer-id", long = "payer-id"))]
    pub payer_id: String,
    #[cfg_attr(feature = "cli", arg(id = "payment-id", long = "payment-id"))]
    pub payment_id: String,
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
