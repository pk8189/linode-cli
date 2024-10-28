#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyVirtModeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "fullvirt"))]
    #[serde(rename = "fullvirt")]
    Fullvirt,
    #[cfg_attr(feature = "cli", value(name = "paravirt"))]
    #[serde(rename = "paravirt")]
    Paravirt,
}
impl std::fmt::Display
for PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyVirtModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyVirtModeEnum::Fullvirt => {
                "fullvirt"
            }
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyVirtModeEnum::Paravirt => {
                "paravirt"
            }
        };
        write!(f, "{}", str_val)
    }
}
