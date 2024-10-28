#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyRunLevelEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "binbash"))]
    #[serde(rename = "binbash")]
    Binbash,
    #[cfg_attr(feature = "cli", value(name = "default"))]
    #[serde(rename = "default")]
    Default,
    #[cfg_attr(feature = "cli", value(name = "single"))]
    #[serde(rename = "single")]
    Single,
}
impl std::fmt::Display
for PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyRunLevelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyRunLevelEnum::Binbash => {
                "binbash"
            }
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyRunLevelEnum::Default => {
                "default"
            }
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyRunLevelEnum::Single => {
                "single"
            }
        };
        write!(f, "{}", str_val)
    }
}
