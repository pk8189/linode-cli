#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "longview-10"))]
    #[serde(rename = "longview-10")]
    Longview10,
    #[cfg_attr(feature = "cli", value(name = "longview-100"))]
    #[serde(rename = "longview-100")]
    Longview100,
    #[cfg_attr(feature = "cli", value(name = "longview-3"))]
    #[serde(rename = "longview-3")]
    Longview3,
    #[cfg_attr(feature = "cli", value(name = "longview-40"))]
    #[serde(rename = "longview-40")]
    Longview40,
}
impl std::fmt::Display for PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum::Longview10 => {
                "longview-10"
            }
            PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum::Longview100 => {
                "longview-100"
            }
            PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum::Longview3 => {
                "longview-3"
            }
            PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum::Longview40 => {
                "longview-40"
            }
        };
        write!(f, "{}", str_val)
    }
}
