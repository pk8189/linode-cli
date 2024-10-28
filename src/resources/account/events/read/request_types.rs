#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionAccountEventsEventIdReadApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "event-id", long = "event-id"))]
    pub event_id: i64,
}
