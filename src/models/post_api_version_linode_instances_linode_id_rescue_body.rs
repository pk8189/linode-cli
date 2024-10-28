#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdRescueBody {
    #[cfg_attr(feature = "cli", arg(id = "devices", long = "devices"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevices>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevices,
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
