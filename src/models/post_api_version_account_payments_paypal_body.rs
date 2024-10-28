#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionAccountPaymentsPaypalBody {
    #[cfg_attr(feature = "cli", arg(id = "cancel-url", long = "cancel-url"))]
    pub cancel_url: String,
    #[cfg_attr(feature = "cli", arg(id = "redirect-url", long = "redirect-url"))]
    pub redirect_url: String,
    #[cfg_attr(feature = "cli", arg(id = "usd", long = "usd"))]
    pub usd: String,
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
