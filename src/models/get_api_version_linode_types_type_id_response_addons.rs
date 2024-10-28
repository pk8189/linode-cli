#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeTypesTypeIdResponseAddons {
    #[cfg_attr(feature = "cli", arg(id = "backups", long = "backups"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeTypesTypeIdResponseAddonsBackups>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<
        crate::models::GetApiVersionLinodeTypesTypeIdResponseAddonsBackups,
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
