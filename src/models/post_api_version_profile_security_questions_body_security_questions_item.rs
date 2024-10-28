#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionProfileSecurityQuestionsBodySecurityQuestionsItem {
    #[cfg_attr(feature = "cli", arg(id = "question-id", long = "question-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "response", long = "response"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "security-question", long = "security-question")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_question: Option<String>,
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
