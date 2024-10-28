#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionAccountBodyActivePromotionsItem {
    #[cfg_attr(
        feature = "cli",
        arg(id = "credit-monthly-cap", long = "credit-monthly-cap")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_monthly_cap: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "credit-remaining", long = "credit-remaining"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_remaining: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "expire-dt", long = "expire-dt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_dt: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "image-url", long = "image-url"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "service-type", long = "service-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<
        crate::models::PutApiVersionAccountBodyActivePromotionsItemServiceTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "summary", long = "summary"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "this-month-credit-remaining", long = "this-month-credit-remaining")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub this_month_credit_remaining: Option<String>,
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
