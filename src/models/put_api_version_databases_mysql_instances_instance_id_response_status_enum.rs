#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "active"))]
    #[serde(rename = "active")]
    Active,
    #[cfg_attr(feature = "cli", value(name = "backing_up"))]
    #[serde(rename = "backing_up")]
    BackingUp,
    #[cfg_attr(feature = "cli", value(name = "degraded"))]
    #[serde(rename = "degraded")]
    Degraded,
    #[cfg_attr(feature = "cli", value(name = "failed"))]
    #[serde(rename = "failed")]
    Failed,
    #[cfg_attr(feature = "cli", value(name = "provisioning"))]
    #[serde(rename = "provisioning")]
    Provisioning,
    #[cfg_attr(feature = "cli", value(name = "restoring"))]
    #[serde(rename = "restoring")]
    Restoring,
    #[cfg_attr(feature = "cli", value(name = "resuming"))]
    #[serde(rename = "resuming")]
    Resuming,
    #[cfg_attr(feature = "cli", value(name = "suspended"))]
    #[serde(rename = "suspended")]
    Suspended,
    #[cfg_attr(feature = "cli", value(name = "suspending"))]
    #[serde(rename = "suspending")]
    Suspending,
    #[cfg_attr(feature = "cli", value(name = "updating"))]
    #[serde(rename = "updating")]
    Updating,
}
impl std::fmt::Display
for PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Active => {
                "active"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::BackingUp => {
                "backing_up"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Degraded => {
                "degraded"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Failed => {
                "failed"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Provisioning => {
                "provisioning"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Restoring => {
                "restoring"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Resuming => {
                "resuming"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Suspended => {
                "suspended"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Suspending => {
                "suspending"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseStatusEnum::Updating => {
                "updating"
            }
        };
        write!(f, "{}", str_val)
    }
}
