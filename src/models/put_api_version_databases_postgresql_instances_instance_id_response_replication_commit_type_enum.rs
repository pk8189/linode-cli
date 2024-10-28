#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum {
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
for PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum::Local => {
                "local"
            }
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum::Off => {
                "off"
            }
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum::On => {
                "on"
            }
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum::RemoteApply => {
                "remote_apply"
            }
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum::RemoteWrite => {
                "remote_write"
            }
        };
        write!(f, "{}", str_val)
    }
}
