#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionLinodeInstancesLinodeIdResizeBodyMigrationTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "cold"))]
    #[serde(rename = "cold")]
    Cold,
    #[cfg_attr(feature = "cli", value(name = "warm"))]
    #[serde(rename = "warm")]
    Warm,
}
impl std::fmt::Display
for PostApiVersionLinodeInstancesLinodeIdResizeBodyMigrationTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionLinodeInstancesLinodeIdResizeBodyMigrationTypeEnum::Cold => {
                "cold"
            }
            PostApiVersionLinodeInstancesLinodeIdResizeBodyMigrationTypeEnum::Warm => {
                "warm"
            }
        };
        write!(f, "{}", str_val)
    }
}
