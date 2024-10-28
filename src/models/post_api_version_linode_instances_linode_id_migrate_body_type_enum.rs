#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionLinodeInstancesLinodeIdMigrateBodyTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "cold"))]
    #[serde(rename = "cold")]
    Cold,
    #[cfg_attr(feature = "cli", value(name = "warm"))]
    #[serde(rename = "warm")]
    Warm,
}
impl std::fmt::Display for PostApiVersionLinodeInstancesLinodeIdMigrateBodyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionLinodeInstancesLinodeIdMigrateBodyTypeEnum::Cold => "cold",
            PostApiVersionLinodeInstancesLinodeIdMigrateBodyTypeEnum::Warm => "warm",
        };
        write!(f, "{}", str_val)
    }
}
