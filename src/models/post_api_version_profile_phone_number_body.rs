#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionProfilePhoneNumberBody {
    #[cfg_attr(feature = "cli", arg(id = "iso-code", long = "iso-code"))]
    pub iso_code: String,
    #[cfg_attr(feature = "cli", arg(id = "phone-number", long = "phone-number"))]
    pub phone_number: String,
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
