#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionRegionsResponseDataItemSiteTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "core"))]
    #[serde(rename = "core")]
    Core,
    #[cfg_attr(feature = "cli", value(name = "distributed"))]
    #[serde(rename = "distributed")]
    Distributed,
}
impl std::fmt::Display for GetApiVersionRegionsResponseDataItemSiteTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionRegionsResponseDataItemSiteTypeEnum::Core => "core",
            GetApiVersionRegionsResponseDataItemSiteTypeEnum::Distributed => {
                "distributed"
            }
        };
        write!(f, "{}", str_val)
    }
}
