#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdatesFrequencyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "monthly"))]
    #[serde(rename = "monthly")]
    Monthly,
    #[cfg_attr(feature = "cli", value(name = "weekly"))]
    #[serde(rename = "weekly")]
    Weekly,
}
impl std::fmt::Display
for PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdatesFrequencyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdatesFrequencyEnum::Monthly => {
                "monthly"
            }
            PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdatesFrequencyEnum::Weekly => {
                "weekly"
            }
        };
        write!(f, "{}", str_val)
    }
}
