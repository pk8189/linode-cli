#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionDatabasesMysqlInstancesInstanceIdCredentialsResetApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "instance-id", long = "instance-id"))]
    pub instance_id: i64,
}
