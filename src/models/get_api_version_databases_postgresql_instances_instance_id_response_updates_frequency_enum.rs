#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseUpdatesFrequencyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "monthly"))]
    #[serde(rename = "monthly")]
    Monthly,
    #[cfg_attr(feature = "cli", value(name = "weekly"))]
    #[serde(rename = "weekly")]
    Weekly,
}
impl std::fmt::Display
for GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseUpdatesFrequencyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseUpdatesFrequencyEnum::Monthly => {
                "monthly"
            }
            GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseUpdatesFrequencyEnum::Weekly => {
                "weekly"
            }
        };
        write!(f, "{}", str_val)
    }
}
