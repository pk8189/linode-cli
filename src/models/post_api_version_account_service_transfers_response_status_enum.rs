#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionAccountServiceTransfersResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "accepted"))]
    #[serde(rename = "accepted")]
    Accepted,
    #[cfg_attr(feature = "cli", value(name = "canceled"))]
    #[serde(rename = "canceled")]
    Canceled,
    #[cfg_attr(feature = "cli", value(name = "completed"))]
    #[serde(rename = "completed")]
    Completed,
    #[cfg_attr(feature = "cli", value(name = "failed"))]
    #[serde(rename = "failed")]
    Failed,
    #[cfg_attr(feature = "cli", value(name = "pending"))]
    #[serde(rename = "pending")]
    Pending,
    #[cfg_attr(feature = "cli", value(name = "stale"))]
    #[serde(rename = "stale")]
    Stale,
}
impl std::fmt::Display for PostApiVersionAccountServiceTransfersResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionAccountServiceTransfersResponseStatusEnum::Accepted => {
                "accepted"
            }
            PostApiVersionAccountServiceTransfersResponseStatusEnum::Canceled => {
                "canceled"
            }
            PostApiVersionAccountServiceTransfersResponseStatusEnum::Completed => {
                "completed"
            }
            PostApiVersionAccountServiceTransfersResponseStatusEnum::Failed => "failed",
            PostApiVersionAccountServiceTransfersResponseStatusEnum::Pending => "pending",
            PostApiVersionAccountServiceTransfersResponseStatusEnum::Stale => "stale",
        };
        write!(f, "{}", str_val)
    }
}
