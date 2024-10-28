#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeTypesResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "addons", long = "addons"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeTypesResponseDataItemAddons>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<crate::models::GetApiVersionLinodeTypesResponseDataItemAddons>,
    #[cfg_attr(feature = "cli", arg(id = "class", long = "class"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<crate::models::GetApiVersionLinodeTypesResponseDataItemClassEnum>,
    #[cfg_attr(feature = "cli", arg(id = "disk", long = "disk"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "gpus", long = "gpus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpus: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "memory", long = "memory"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "network-out", long = "network-out"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_out: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "price", long = "price"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeTypesResponseDataItemPrice>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::models::GetApiVersionLinodeTypesResponseDataItemPrice>,
    #[cfg_attr(feature = "cli", arg(id = "region-prices", long = "region-prices"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeTypesResponseDataItemRegionPricesItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_prices: Option<
        Vec<crate::models::GetApiVersionLinodeTypesResponseDataItemRegionPricesItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "successor", long = "successor"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub successor: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "transfer", long = "transfer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "vcpus", long = "vcpus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
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
