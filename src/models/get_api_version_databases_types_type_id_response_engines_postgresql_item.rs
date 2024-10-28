#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionDatabasesTypesTypeIdResponseEnginesPostgresqlItem {
    #[cfg_attr(feature = "cli", arg(id = "price", long = "price"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionDatabasesTypesTypeIdResponseEnginesPostgresqlItemPrice>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<
        crate::models::GetApiVersionDatabasesTypesTypeIdResponseEnginesPostgresqlItemPrice,
    >,
    #[cfg_attr(feature = "cli", arg(id = "quantity", long = "quantity"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
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
