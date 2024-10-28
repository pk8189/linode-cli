#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionProfileDevicesDeviceIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "expiry", long = "expiry"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "last-authenticated", long = "last-authenticated")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authenticated: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "last-remote-addr", long = "last-remote-addr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_remote_addr: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "user-agent", long = "user-agent"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
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
