#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionDatabasesMysqlInstancesResponseUpdates {
    #[cfg_attr(feature = "cli", arg(id = "day-of-week", long = "day-of-week"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "duration", long = "duration"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "frequency", long = "frequency"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<
        crate::models::PostApiVersionDatabasesMysqlInstancesResponseUpdatesFrequencyEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "hour-of-day", long = "hour-of-day"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour_of_day: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "week-of-month", long = "week-of-month"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub week_of_month: crate::core::patch::Patch<i64>,
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
