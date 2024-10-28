#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBodyTargetEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "primary"))]
    #[serde(rename = "primary")]
    Primary,
    #[cfg_attr(feature = "cli", value(name = "secondary"))]
    #[serde(rename = "secondary")]
    Secondary,
}
impl std::fmt::Display
for PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBodyTargetEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBodyTargetEnum::Primary => {
                "primary"
            }
            PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBodyTargetEnum::Secondary => {
                "secondary"
            }
        };
        write!(f, "{}", str_val)
    }
}
