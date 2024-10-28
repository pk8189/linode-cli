#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "fullvirt"))]
    #[serde(rename = "fullvirt")]
    Fullvirt,
    #[cfg_attr(feature = "cli", value(name = "paravirt"))]
    #[serde(rename = "paravirt")]
    Paravirt,
}
impl std::fmt::Display
for PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum::Fullvirt => {
                "fullvirt"
            }
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseVirtModeEnum::Paravirt => {
                "paravirt"
            }
        };
        write!(f, "{}", str_val)
    }
}
