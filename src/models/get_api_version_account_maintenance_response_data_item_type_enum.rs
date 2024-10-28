#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountMaintenanceResponseDataItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "cold_migration"))]
    #[serde(rename = "cold_migration")]
    ColdMigration,
    #[cfg_attr(feature = "cli", value(name = "live_migration"))]
    #[serde(rename = "live_migration")]
    LiveMigration,
    #[cfg_attr(feature = "cli", value(name = "reboot"))]
    #[serde(rename = "reboot")]
    Reboot,
}
impl std::fmt::Display for GetApiVersionAccountMaintenanceResponseDataItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountMaintenanceResponseDataItemTypeEnum::ColdMigration => {
                "cold_migration"
            }
            GetApiVersionAccountMaintenanceResponseDataItemTypeEnum::LiveMigration => {
                "live_migration"
            }
            GetApiVersionAccountMaintenanceResponseDataItemTypeEnum::Reboot => "reboot",
        };
        write!(f, "{}", str_val)
    }
}
