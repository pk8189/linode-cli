#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj2 {
    #[cfg_attr(feature = "cli", arg(id = "email", long = "email"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "paypal-id", long = "paypal-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal_id: Option<String>,
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
