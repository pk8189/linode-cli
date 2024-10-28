#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountEventsResponseDataItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "failed"))]
    #[serde(rename = "failed")]
    Failed,
    #[cfg_attr(feature = "cli", value(name = "finished"))]
    #[serde(rename = "finished")]
    Finished,
    #[cfg_attr(feature = "cli", value(name = "notification"))]
    #[serde(rename = "notification")]
    Notification,
    #[cfg_attr(feature = "cli", value(name = "scheduled"))]
    #[serde(rename = "scheduled")]
    Scheduled,
    #[cfg_attr(feature = "cli", value(name = "started"))]
    #[serde(rename = "started")]
    Started,
}
impl std::fmt::Display for GetApiVersionAccountEventsResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountEventsResponseDataItemStatusEnum::Failed => "failed",
            GetApiVersionAccountEventsResponseDataItemStatusEnum::Finished => "finished",
            GetApiVersionAccountEventsResponseDataItemStatusEnum::Notification => {
                "notification"
            }
            GetApiVersionAccountEventsResponseDataItemStatusEnum::Scheduled => {
                "scheduled"
            }
            GetApiVersionAccountEventsResponseDataItemStatusEnum::Started => "started",
        };
        write!(f, "{}", str_val)
    }
}
