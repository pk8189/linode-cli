#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupPolicyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "flexible"))]
    #[serde(rename = "flexible")]
    Flexible,
    #[cfg_attr(feature = "cli", value(name = "strict"))]
    #[serde(rename = "strict")]
    Strict,
}
impl std::fmt::Display
for PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupPolicyEnum::Flexible => {
                "flexible"
            }
            PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupPolicyEnum::Strict => {
                "strict"
            }
        };
        write!(f, "{}", str_val)
    }
}
