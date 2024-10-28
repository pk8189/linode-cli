#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionAccountAgreementsBody {
    #[cfg_attr(feature = "cli", arg(id = "eu-model", long = "eu-model"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_model: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "master-service-agreement", long = "master-service-agreement")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_service_agreement: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "privacy-policy", long = "privacy-policy"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<bool>,
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
