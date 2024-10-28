#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "Scheduling"))]
    #[serde(rename = "Scheduling")]
    Scheduling,
    #[cfg_attr(feature = "cli", value(name = "W0"))]
    #[serde(rename = "W0")]
    W0,
    #[cfg_attr(feature = "cli", value(name = "W10"))]
    #[serde(rename = "W10")]
    W10,
    #[cfg_attr(feature = "cli", value(name = "W12"))]
    #[serde(rename = "W12")]
    W12,
    #[cfg_attr(feature = "cli", value(name = "W14"))]
    #[serde(rename = "W14")]
    W14,
    #[cfg_attr(feature = "cli", value(name = "W16"))]
    #[serde(rename = "W16")]
    W16,
    #[cfg_attr(feature = "cli", value(name = "W18"))]
    #[serde(rename = "W18")]
    W18,
    #[cfg_attr(feature = "cli", value(name = "W2"))]
    #[serde(rename = "W2")]
    W2,
    #[cfg_attr(feature = "cli", value(name = "W20"))]
    #[serde(rename = "W20")]
    W20,
    #[cfg_attr(feature = "cli", value(name = "W22"))]
    #[serde(rename = "W22")]
    W22,
    #[cfg_attr(feature = "cli", value(name = "W4"))]
    #[serde(rename = "W4")]
    W4,
    #[cfg_attr(feature = "cli", value(name = "W6"))]
    #[serde(rename = "W6")]
    W6,
    #[cfg_attr(feature = "cli", value(name = "W8"))]
    #[serde(rename = "W8")]
    W8,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::Scheduling => {
                "Scheduling"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W0 => {
                "W0"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W10 => {
                "W10"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W12 => {
                "W12"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W14 => {
                "W14"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W16 => {
                "W16"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W18 => {
                "W18"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W2 => {
                "W2"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W20 => {
                "W20"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W22 => {
                "W22"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W4 => {
                "W4"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W6 => {
                "W6"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseBackupsScheduleWindowEnum::W8 => {
                "W8"
            }
        };
        write!(f, "{}", str_val)
    }
}
