#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeTypesResponseDataItemAddonsBackups {
    #[cfg_attr(feature = "cli", arg(id = "price", long = "price"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeTypesResponseDataItemAddonsBackupsPrice>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<
        crate::models::GetApiVersionLinodeTypesResponseDataItemAddonsBackupsPrice,
    >,
    #[cfg_attr(feature = "cli", arg(id = "region-prices", long = "region-prices"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeTypesResponseDataItemAddonsBackupsRegionPricesItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_prices: Option<
        Vec<
            crate::models::GetApiVersionLinodeTypesResponseDataItemAddonsBackupsRegionPricesItem,
        >,
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
