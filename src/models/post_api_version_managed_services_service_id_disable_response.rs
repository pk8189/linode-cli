#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionManagedServicesServiceIdDisableResponse {
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "body", long = "body"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub body: crate::core::patch::Patch<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "consultation-group", long = "consultation-group")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consultation_group: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "credentials", long = "credentials"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<i64>>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "notes", long = "notes"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub notes: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "service-type", long = "service-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<
        crate::models::PostApiVersionManagedServicesServiceIdDisableResponseServiceTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::PostApiVersionManagedServicesServiceIdDisableResponseStatusEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "timeout", long = "timeout"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
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
