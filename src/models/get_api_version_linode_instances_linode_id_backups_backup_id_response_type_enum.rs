#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "auto"))]
    #[serde(rename = "auto")]
    Auto,
    #[cfg_attr(feature = "cli", value(name = "snapshot"))]
    #[serde(rename = "snapshot")]
    Snapshot,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseTypeEnum::Auto => {
                "auto"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdResponseTypeEnum::Snapshot => {
                "snapshot"
            }
        };
        write!(f, "{}", str_val)
    }
}
