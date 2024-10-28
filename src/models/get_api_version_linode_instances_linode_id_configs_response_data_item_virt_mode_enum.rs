#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemVirtModeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "fullvirt"))]
    #[serde(rename = "fullvirt")]
    Fullvirt,
    #[cfg_attr(feature = "cli", value(name = "paravirt"))]
    #[serde(rename = "paravirt")]
    Paravirt,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemVirtModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemVirtModeEnum::Fullvirt => {
                "fullvirt"
            }
            GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemVirtModeEnum::Paravirt => {
                "paravirt"
            }
        };
        write!(f, "{}", str_val)
    }
}
