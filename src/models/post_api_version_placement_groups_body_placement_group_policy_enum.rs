#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionPlacementGroupsBodyPlacementGroupPolicyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "flexible"))]
    #[serde(rename = "flexible")]
    Flexible,
    #[cfg_attr(feature = "cli", value(name = "strict"))]
    #[serde(rename = "strict")]
    Strict,
}
impl std::fmt::Display for PostApiVersionPlacementGroupsBodyPlacementGroupPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionPlacementGroupsBodyPlacementGroupPolicyEnum::Flexible => {
                "flexible"
            }
            PostApiVersionPlacementGroupsBodyPlacementGroupPolicyEnum::Strict => "strict",
        };
        write!(f, "{}", str_val)
    }
}
