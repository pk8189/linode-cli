#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "failed"))]
    #[serde(rename = "failed")]
    Failed,
    #[cfg_attr(feature = "cli", value(name = "needsPostProcessing"))]
    #[serde(rename = "needsPostProcessing")]
    NeedsPostProcessing,
    #[cfg_attr(feature = "cli", value(name = "paused"))]
    #[serde(rename = "paused")]
    Paused,
    #[cfg_attr(feature = "cli", value(name = "pending"))]
    #[serde(rename = "pending")]
    Pending,
    #[cfg_attr(feature = "cli", value(name = "running"))]
    #[serde(rename = "running")]
    Running,
    #[cfg_attr(feature = "cli", value(name = "successful"))]
    #[serde(rename = "successful")]
    Successful,
    #[cfg_attr(feature = "cli", value(name = "userAborted"))]
    #[serde(rename = "userAborted")]
    UserAborted,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum::Failed => {
                "failed"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum::NeedsPostProcessing => {
                "needsPostProcessing"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum::Paused => {
                "paused"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum::Pending => {
                "pending"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum::Running => {
                "running"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum::Successful => {
                "successful"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressStatusEnum::UserAborted => {
                "userAborted"
            }
        };
        write!(f, "{}", str_val)
    }
}
