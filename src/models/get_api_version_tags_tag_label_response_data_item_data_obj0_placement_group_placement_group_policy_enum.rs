#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionTagsTagLabelResponseDataItemDataObj0PlacementGroupPlacementGroupPolicyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "flexible"))]
    #[serde(rename = "flexible")]
    Flexible,
    #[cfg_attr(feature = "cli", value(name = "strict"))]
    #[serde(rename = "strict")]
    Strict,
}
impl std::fmt::Display
for GetApiVersionTagsTagLabelResponseDataItemDataObj0PlacementGroupPlacementGroupPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionTagsTagLabelResponseDataItemDataObj0PlacementGroupPlacementGroupPolicyEnum::Flexible => {
                "flexible"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0PlacementGroupPlacementGroupPolicyEnum::Strict => {
                "strict"
            }
        };
        write!(f, "{}", str_val)
    }
}
