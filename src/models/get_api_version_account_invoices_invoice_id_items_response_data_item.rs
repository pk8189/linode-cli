#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountInvoicesInvoiceIdItemsResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "amount", long = "amount"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "from", long = "from"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "quantity", long = "quantity"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub region: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "tax", long = "tax"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "to", long = "to"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "total", long = "total"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<
        crate::models::GetApiVersionAccountInvoicesInvoiceIdItemsResponseDataItemTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "unit-price", long = "unit-price"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
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
