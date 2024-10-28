#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionObjectStorageClustersClusterIdResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "available"))]
    #[serde(rename = "available")]
    Available,
    #[cfg_attr(feature = "cli", value(name = "unavailable"))]
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl std::fmt::Display
for GetApiVersionObjectStorageClustersClusterIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionObjectStorageClustersClusterIdResponseStatusEnum::Available => {
                "available"
            }
            GetApiVersionObjectStorageClustersClusterIdResponseStatusEnum::Unavailable => {
                "unavailable"
            }
        };
        write!(f, "{}", str_val)
    }
}
