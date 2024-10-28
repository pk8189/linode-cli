#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "asynch"))]
    #[serde(rename = "asynch")]
    Asynch,
    #[cfg_attr(feature = "cli", value(name = "none"))]
    #[serde(rename = "none")]
    None,
}
impl std::fmt::Display
for PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationTypeEnum::Asynch => {
                "asynch"
            }
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationTypeEnum::None => {
                "none"
            }
        };
        write!(f, "{}", str_val)
    }
}
