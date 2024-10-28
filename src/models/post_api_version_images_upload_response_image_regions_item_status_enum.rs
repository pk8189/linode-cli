#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "available"))]
    #[serde(rename = "available")]
    Available,
    #[cfg_attr(feature = "cli", value(name = "creating"))]
    #[serde(rename = "creating")]
    Creating,
    #[cfg_attr(feature = "cli", value(name = "pending"))]
    #[serde(rename = "pending")]
    Pending,
    #[cfg_attr(feature = "cli", value(name = "pending deletion"))]
    #[serde(rename = "pending deletion")]
    PendingDeletion,
    #[cfg_attr(feature = "cli", value(name = "pending replication"))]
    #[serde(rename = "pending replication")]
    PendingReplication,
    #[cfg_attr(feature = "cli", value(name = "replicating"))]
    #[serde(rename = "replicating")]
    Replicating,
}
impl std::fmt::Display for PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum::Available => {
                "available"
            }
            PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum::Creating => {
                "creating"
            }
            PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum::Pending => {
                "pending"
            }
            PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum::PendingDeletion => {
                "pending deletion"
            }
            PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum::PendingReplication => {
                "pending replication"
            }
            PostApiVersionImagesUploadResponseImageRegionsItemStatusEnum::Replicating => {
                "replicating"
            }
        };
        write!(f, "{}", str_val)
    }
}
