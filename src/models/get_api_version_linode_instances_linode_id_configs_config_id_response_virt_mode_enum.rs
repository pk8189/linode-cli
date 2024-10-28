#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "fullvirt"))]
    #[serde(rename = "fullvirt")]
    Fullvirt,
    #[cfg_attr(feature = "cli", value(name = "paravirt"))]
    #[serde(rename = "paravirt")]
    Paravirt,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum::Fullvirt => {
                "fullvirt"
            }
            GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum::Paravirt => {
                "paravirt"
            }
        };
        write!(f, "{}", str_val)
    }
}
