#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionDatabasesMysqlInstancesResponseReplicationTypeEnum {
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
for PostApiVersionDatabasesMysqlInstancesResponseReplicationTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionDatabasesMysqlInstancesResponseReplicationTypeEnum::Asynch => {
                "asynch"
            }
            PostApiVersionDatabasesMysqlInstancesResponseReplicationTypeEnum::None => {
                "none"
            }
            PostApiVersionDatabasesMysqlInstancesResponseReplicationTypeEnum::SemiSynch => {
                "semi_synch"
            }
        };
        write!(f, "{}", str_val)
    }
}
