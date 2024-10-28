#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionManagedLinodeSettingsLinodeIdResponseSsh {
    #[cfg_attr(feature = "cli", arg(id = "access", long = "access"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "ip", long = "ip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "port", long = "port"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub port: crate::core::patch::Patch<i64>,
    #[cfg_attr(feature = "cli", arg(id = "user", long = "user"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub user: crate::core::patch::Patch<String>,
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
