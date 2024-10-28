#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionAccountEntityTransfersResponse {
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "entities", long = "entities"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionAccountEntityTransfersResponseEntities>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<
        crate::models::PostApiVersionAccountEntityTransfersResponseEntities,
    >,
    #[cfg_attr(feature = "cli", arg(id = "expiry", long = "expiry"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "is-sender", long = "is-sender"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sender: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::PostApiVersionAccountEntityTransfersResponseStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "token", long = "token"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
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
