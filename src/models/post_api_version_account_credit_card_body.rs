#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionAccountCreditCardBody {
    #[cfg_attr(feature = "cli", arg(id = "card-number", long = "card-number"))]
    pub card_number: String,
    #[cfg_attr(feature = "cli", arg(id = "cvv", long = "cvv"))]
    pub cvv: String,
    #[cfg_attr(feature = "cli", arg(id = "expiry-month", long = "expiry-month"))]
    pub expiry_month: i64,
    #[cfg_attr(feature = "cli", arg(id = "expiry-year", long = "expiry-year"))]
    pub expiry_year: i64,
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
