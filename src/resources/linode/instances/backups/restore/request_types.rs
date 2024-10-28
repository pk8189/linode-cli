#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionLinodeInstancesLinodeIdBackupsBackupIdRestoreApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "backup-id", long = "backup-id"))]
    pub backup_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionLinodeInstancesLinodeIdBackupsBackupIdRestoreBody,
}
