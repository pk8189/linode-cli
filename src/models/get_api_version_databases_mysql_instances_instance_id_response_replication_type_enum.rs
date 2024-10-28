#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionDatabasesMysqlInstancesInstanceIdResponseReplicationTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "asynch"))]
    #[serde(rename = "asynch")]
    Asynch,
    #[cfg_attr(feature = "cli", value(name = "none"))]
    #[serde(rename = "none")]
    None,
    #[cfg_attr(feature = "cli", value(name = "semi_synch"))]
    #[serde(rename = "semi_synch")]
    SemiSynch,
}
impl std::fmt::Display
for GetApiVersionDatabasesMysqlInstancesInstanceIdResponseReplicationTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionDatabasesMysqlInstancesInstanceIdResponseReplicationTypeEnum::Asynch => {
                "asynch"
            }
            GetApiVersionDatabasesMysqlInstancesInstanceIdResponseReplicationTypeEnum::None => {
                "none"
            }
            GetApiVersionDatabasesMysqlInstancesInstanceIdResponseReplicationTypeEnum::SemiSynch => {
                "semi_synch"
            }
        };
        write!(f, "{}", str_val)
    }
}
