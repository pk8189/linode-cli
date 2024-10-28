#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionDatabasesPostgresqlInstancesResponseReplicationCommitTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "local"))]
    #[serde(rename = "local")]
    Local,
    #[cfg_attr(feature = "cli", value(name = "off"))]
    #[serde(rename = "off")]
    Off,
    #[cfg_attr(feature = "cli", value(name = "on"))]
    #[serde(rename = "on")]
    On,
    #[cfg_attr(feature = "cli", value(name = "remote_apply"))]
    #[serde(rename = "remote_apply")]
    RemoteApply,
    #[cfg_attr(feature = "cli", value(name = "remote_write"))]
    #[serde(rename = "remote_write")]
    RemoteWrite,
}
impl std::fmt::Display
for PostApiVersionDatabasesPostgresqlInstancesResponseReplicationCommitTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionDatabasesPostgresqlInstancesResponseReplicationCommitTypeEnum::Local => {
                "local"
            }
            PostApiVersionDatabasesPostgresqlInstancesResponseReplicationCommitTypeEnum::Off => {
                "off"
            }
            PostApiVersionDatabasesPostgresqlInstancesResponseReplicationCommitTypeEnum::On => {
                "on"
            }
            PostApiVersionDatabasesPostgresqlInstancesResponseReplicationCommitTypeEnum::RemoteApply => {
                "remote_apply"
            }
            PostApiVersionDatabasesPostgresqlInstancesResponseReplicationCommitTypeEnum::RemoteWrite => {
                "remote_write"
            }
        };
        write!(f, "{}", str_val)
    }
}
