#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionObjectStorageTypesResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "price", long = "price"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionObjectStorageTypesResponseDataItemPrice>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<
        crate::models::GetApiVersionObjectStorageTypesResponseDataItemPrice,
    >,
    #[cfg_attr(feature = "cli", arg(id = "region-prices", long = "region-prices"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionObjectStorageTypesResponseDataItemRegionPricesItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_prices: Option<
        Vec<
            crate::models::GetApiVersionObjectStorageTypesResponseDataItemRegionPricesItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "transfer", long = "transfer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<i64>,
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
