#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLinodeInstancesLinodeIdResponseHypervisorEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "kvm"))]
    #[serde(rename = "kvm")]
    Kvm,
}
impl std::fmt::Display for PutApiVersionLinodeInstancesLinodeIdResponseHypervisorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLinodeInstancesLinodeIdResponseHypervisorEnum::Kvm => "kvm",
        };
        write!(f, "{}", str_val)
    }
}
