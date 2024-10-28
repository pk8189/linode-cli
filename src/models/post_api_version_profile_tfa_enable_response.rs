#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionProfileTfaEnableResponse {
    #[cfg_attr(feature = "cli", arg(id = "expiry", long = "expiry"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "secret", long = "secret"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
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
