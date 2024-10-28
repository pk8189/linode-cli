#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionLinodeInstancesLinodeIdConfigsBodyVirtModeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "fullvirt"))]
    #[serde(rename = "fullvirt")]
    Fullvirt,
    #[cfg_attr(feature = "cli", value(name = "paravirt"))]
    #[serde(rename = "paravirt")]
    Paravirt,
}
impl std::fmt::Display for PostApiVersionLinodeInstancesLinodeIdConfigsBodyVirtModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionLinodeInstancesLinodeIdConfigsBodyVirtModeEnum::Fullvirt => {
                "fullvirt"
            }
            PostApiVersionLinodeInstancesLinodeIdConfigsBodyVirtModeEnum::Paravirt => {
                "paravirt"
            }
        };
        write!(f, "{}", str_val)
    }
}
