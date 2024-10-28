#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBackupIdRestoreApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "instance-id", long = "instance-id"))]
    pub instance_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "backup-id", long = "backup-id"))]
    pub backup_id: i64,
}
