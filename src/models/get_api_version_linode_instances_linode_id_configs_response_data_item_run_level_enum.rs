#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemRunLevelEnum {
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
for GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemRunLevelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemRunLevelEnum::Binbash => {
                "binbash"
            }
            GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemRunLevelEnum::Default => {
                "default"
            }
            GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemRunLevelEnum::Single => {
                "single"
            }
        };
        write!(f, "{}", str_val)
    }
}
