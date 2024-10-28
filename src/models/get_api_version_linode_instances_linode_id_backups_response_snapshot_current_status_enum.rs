#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum {
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
for GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum::Failed => {
                "failed"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum::NeedsPostProcessing => {
                "needsPostProcessing"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum::Paused => {
                "paused"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum::Pending => {
                "pending"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum::Running => {
                "running"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum::Successful => {
                "successful"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotCurrentStatusEnum::UserAborted => {
                "userAborted"
            }
        };
        write!(f, "{}", str_val)
    }
}
