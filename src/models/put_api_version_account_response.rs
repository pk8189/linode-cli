#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionAccountResponse {
    #[cfg_attr(
        feature = "cli",
        arg(id = "active-promotions", long = "active-promotions")
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountResponseActivePromotionsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_promotions: Option<
        Vec<crate::models::PutApiVersionAccountResponseActivePromotionsItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "active-since", long = "active-since"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_since: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "address-1", long = "address-1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "address-2", long = "address-2"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "balance", long = "balance"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "balance-uninvoiced", long = "balance-uninvoiced")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_uninvoiced: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "billing-source", long = "billing-source"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_source: Option<
        crate::models::PutApiVersionAccountResponseBillingSourceEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "capabilities", long = "capabilities"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "city", long = "city"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "company", long = "company"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "country", long = "country"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "credit-card", long = "credit-card"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountResponseCreditCard>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<crate::models::PutApiVersionAccountResponseCreditCard>,
    #[cfg_attr(feature = "cli", arg(id = "email", long = "email"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "euuid", long = "euuid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub euuid: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "first-name", long = "first-name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "last-name", long = "last-name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "phone", long = "phone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "state", long = "state"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "tax-id", long = "tax-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "zip", long = "zip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
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
