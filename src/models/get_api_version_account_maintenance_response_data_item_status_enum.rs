#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountMaintenanceResponseDataItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "completed"))]
    #[serde(rename = "completed")]
    Completed,
    #[cfg_attr(feature = "cli", value(name = "pending"))]
    #[serde(rename = "pending")]
    Pending,
    #[cfg_attr(feature = "cli", value(name = "started"))]
    #[serde(rename = "started")]
    Started,
}
impl std::fmt::Display for GetApiVersionAccountMaintenanceResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountMaintenanceResponseDataItemStatusEnum::Completed => {
                "completed"
            }
            GetApiVersionAccountMaintenanceResponseDataItemStatusEnum::Pending => {
                "pending"
            }
            GetApiVersionAccountMaintenanceResponseDataItemStatusEnum::Started => {
                "started"
            }
        };
        write!(f, "{}", str_val)
    }
}
