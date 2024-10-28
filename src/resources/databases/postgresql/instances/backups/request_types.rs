#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "instance-id", long = "instance-id"))]
    pub instance_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "backup-id", long = "backup-id"))]
    pub backup_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "instance-id", long = "instance-id"))]
    pub instance_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "page", long = "page"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "page-size", long = "page-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "instance-id", long = "instance-id"))]
    pub instance_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "backup-id", long = "backup-id"))]
    pub backup_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "instance-id", long = "instance-id"))]
    pub instance_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBody,
}
