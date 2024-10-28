#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountInvoicesResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "billing-source", long = "billing-source"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_source: Option<
        crate::models::GetApiVersionAccountInvoicesResponseDataItemBillingSourceEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "date", long = "date"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "subtotal", long = "subtotal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "tax", long = "tax"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "tax-summary", long = "tax-summary"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionAccountInvoicesResponseDataItemTaxSummaryItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_summary: Option<
        Vec<crate::models::GetApiVersionAccountInvoicesResponseDataItemTaxSummaryItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "total", long = "total"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
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
