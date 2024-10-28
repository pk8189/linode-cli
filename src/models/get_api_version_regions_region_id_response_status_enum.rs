#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionRegionsRegionIdResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ok"))]
    #[serde(rename = "ok")]
    Ok,
    #[cfg_attr(feature = "cli", value(name = "outage"))]
    #[serde(rename = "outage")]
    Outage,
}
impl std::fmt::Display for GetApiVersionRegionsRegionIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionRegionsRegionIdResponseStatusEnum::Ok => "ok",
            GetApiVersionRegionsRegionIdResponseStatusEnum::Outage => "outage",
        };
        write!(f, "{}", str_val)
    }
}
