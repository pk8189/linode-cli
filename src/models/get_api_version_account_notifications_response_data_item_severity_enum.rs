#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountNotificationsResponseDataItemSeverityEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "critical"))]
    #[serde(rename = "critical")]
    Critical,
    #[cfg_attr(feature = "cli", value(name = "major"))]
    #[serde(rename = "major")]
    Major,
    #[cfg_attr(feature = "cli", value(name = "minor"))]
    #[serde(rename = "minor")]
    Minor,
}
impl std::fmt::Display
for GetApiVersionAccountNotificationsResponseDataItemSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountNotificationsResponseDataItemSeverityEnum::Critical => {
                "critical"
            }
            GetApiVersionAccountNotificationsResponseDataItemSeverityEnum::Major => {
                "major"
            }
            GetApiVersionAccountNotificationsResponseDataItemSeverityEnum::Minor => {
                "minor"
            }
        };
        write!(f, "{}", str_val)
    }
}
