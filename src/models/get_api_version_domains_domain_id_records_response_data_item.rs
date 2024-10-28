#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionDomainsDomainIdRecordsResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "name", long = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "port", long = "port"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "priority", long = "priority"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "protocol", long = "protocol"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub protocol: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "service", long = "service"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub service: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "tag", long = "tag"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<crate::models::GetApiVersionDomainsDomainIdRecordsResponseDataItemTagEnum>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub tag: crate::core::patch::Patch<
        crate::models::GetApiVersionDomainsDomainIdRecordsResponseDataItemTagEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "target", long = "target"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "ttl-sec", long = "ttl-sec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_sec: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<
        crate::models::GetApiVersionDomainsDomainIdRecordsResponseDataItemTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "weight", long = "weight"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
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
