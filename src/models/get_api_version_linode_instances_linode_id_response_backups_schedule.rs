#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdResponseBackupsSchedule {
    #[cfg_attr(feature = "cli", arg(id = "day", long = "day"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<crate::models::GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleDayEnum>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub day: crate::core::patch::Patch<
        crate::models::GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleDayEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "window", long = "window"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<crate::models::GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub window: crate::core::patch::Patch<
        crate::models::GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum,
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
