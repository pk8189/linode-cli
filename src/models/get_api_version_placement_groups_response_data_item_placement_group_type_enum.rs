#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionPlacementGroupsResponseDataItemPlacementGroupTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "anti_affinity:local"))]
    #[serde(rename = "anti_affinity:local")]
    AntiAffinityLocal,
}
impl std::fmt::Display
for GetApiVersionPlacementGroupsResponseDataItemPlacementGroupTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionPlacementGroupsResponseDataItemPlacementGroupTypeEnum::AntiAffinityLocal => {
                "anti_affinity:local"
            }
        };
        write!(f, "{}", str_val)
    }
}
