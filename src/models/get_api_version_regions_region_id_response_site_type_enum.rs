#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionRegionsRegionIdResponseSiteTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "core"))]
    #[serde(rename = "core")]
    Core,
    #[cfg_attr(feature = "cli", value(name = "distributed"))]
    #[serde(rename = "distributed")]
    Distributed,
}
impl std::fmt::Display for GetApiVersionRegionsRegionIdResponseSiteTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionRegionsRegionIdResponseSiteTypeEnum::Core => "core",
            GetApiVersionRegionsRegionIdResponseSiteTypeEnum::Distributed => {
                "distributed"
            }
        };
        write!(f, "{}", str_val)
    }
}
