#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLkeClustersClusterIdPoolsResponseDataItemTaintsItemEffectEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "NoExecute"))]
    #[serde(rename = "NoExecute")]
    NoExecute,
    #[cfg_attr(feature = "cli", value(name = "NoSchedule"))]
    #[serde(rename = "NoSchedule")]
    NoSchedule,
    #[cfg_attr(feature = "cli", value(name = "PreferNoSchedule"))]
    #[serde(rename = "PreferNoSchedule")]
    PreferNoSchedule,
}
impl std::fmt::Display
for GetApiVersionLkeClustersClusterIdPoolsResponseDataItemTaintsItemEffectEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLkeClustersClusterIdPoolsResponseDataItemTaintsItemEffectEnum::NoExecute => {
                "NoExecute"
            }
            GetApiVersionLkeClustersClusterIdPoolsResponseDataItemTaintsItemEffectEnum::NoSchedule => {
                "NoSchedule"
            }
            GetApiVersionLkeClustersClusterIdPoolsResponseDataItemTaintsItemEffectEnum::PreferNoSchedule => {
                "PreferNoSchedule"
            }
        };
        write!(f, "{}", str_val)
    }
}
