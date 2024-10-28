#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionObjectStorageClustersResponseDataItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "available"))]
    #[serde(rename = "available")]
    Available,
    #[cfg_attr(feature = "cli", value(name = "unavailable"))]
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl std::fmt::Display for GetApiVersionObjectStorageClustersResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionObjectStorageClustersResponseDataItemStatusEnum::Available => {
                "available"
            }
            GetApiVersionObjectStorageClustersResponseDataItemStatusEnum::Unavailable => {
                "unavailable"
            }
        };
        write!(f, "{}", str_val)
    }
}
