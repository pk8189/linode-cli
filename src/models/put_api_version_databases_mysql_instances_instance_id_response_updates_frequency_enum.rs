#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionDatabasesMysqlInstancesInstanceIdResponseUpdatesFrequencyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "monthly"))]
    #[serde(rename = "monthly")]
    Monthly,
    #[cfg_attr(feature = "cli", value(name = "weekly"))]
    #[serde(rename = "weekly")]
    Weekly,
}
impl std::fmt::Display
for PutApiVersionDatabasesMysqlInstancesInstanceIdResponseUpdatesFrequencyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseUpdatesFrequencyEnum::Monthly => {
                "monthly"
            }
            PutApiVersionDatabasesMysqlInstancesInstanceIdResponseUpdatesFrequencyEnum::Weekly => {
                "weekly"
            }
        };
        write!(f, "{}", str_val)
    }
}
