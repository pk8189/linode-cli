#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLongviewSubscriptionsSubscriptionIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "clients-included", long = "clients-included"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients_included: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<
        crate::models::GetApiVersionLongviewSubscriptionsSubscriptionIdResponseIdEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "price", long = "price"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLongviewSubscriptionsSubscriptionIdResponsePrice>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<
        crate::models::GetApiVersionLongviewSubscriptionsSubscriptionIdResponsePrice,
    >,
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
