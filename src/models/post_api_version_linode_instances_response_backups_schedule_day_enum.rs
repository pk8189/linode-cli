#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum {
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
impl std::fmt::Display for PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Friday => {
                "Friday"
            }
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Monday => {
                "Monday"
            }
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Saturday => {
                "Saturday"
            }
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Scheduling => {
                "Scheduling"
            }
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Sunday => {
                "Sunday"
            }
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Thursday => {
                "Thursday"
            }
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Tuesday => {
                "Tuesday"
            }
            PostApiVersionLinodeInstancesResponseBackupsScheduleDayEnum::Wednesday => {
                "Wednesday"
            }
        };
        write!(f, "{}", str_val)
    }
}
