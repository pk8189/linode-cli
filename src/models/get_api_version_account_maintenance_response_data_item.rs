#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionAccountMaintenanceResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "entity", long = "entity"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionAccountMaintenanceResponseDataItemEntity>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<
        crate::models::GetApiVersionAccountMaintenanceResponseDataItemEntity,
    >,
    #[cfg_attr(feature = "cli", arg(id = "reason", long = "reason"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::GetApiVersionAccountMaintenanceResponseDataItemStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<
        crate::models::GetApiVersionAccountMaintenanceResponseDataItemTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "when", long = "when"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
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
