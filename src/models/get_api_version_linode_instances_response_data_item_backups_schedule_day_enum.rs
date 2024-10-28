#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "Friday"))]
    #[serde(rename = "Friday")]
    Friday,
    #[cfg_attr(feature = "cli", value(name = "Monday"))]
    #[serde(rename = "Monday")]
    Monday,
    #[cfg_attr(feature = "cli", value(name = "Saturday"))]
    #[serde(rename = "Saturday")]
    Saturday,
    #[cfg_attr(feature = "cli", value(name = "Scheduling"))]
    #[serde(rename = "Scheduling")]
    Scheduling,
    #[cfg_attr(feature = "cli", value(name = "Sunday"))]
    #[serde(rename = "Sunday")]
    Sunday,
    #[cfg_attr(feature = "cli", value(name = "Thursday"))]
    #[serde(rename = "Thursday")]
    Thursday,
    #[cfg_attr(feature = "cli", value(name = "Tuesday"))]
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[cfg_attr(feature = "cli", value(name = "Wednesday"))]
    #[serde(rename = "Wednesday")]
    Wednesday,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Friday => {
                "Friday"
            }
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Monday => {
                "Monday"
            }
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Saturday => {
                "Saturday"
            }
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Scheduling => {
                "Scheduling"
            }
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Sunday => {
                "Sunday"
            }
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Thursday => {
                "Thursday"
            }
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Tuesday => {
                "Tuesday"
            }
            GetApiVersionLinodeInstancesResponseDataItemBackupsScheduleDayEnum::Wednesday => {
                "Wednesday"
            }
        };
        write!(f, "{}", str_val)
    }
}
