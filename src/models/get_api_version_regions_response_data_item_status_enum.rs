#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionRegionsResponseDataItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ok"))]
    #[serde(rename = "ok")]
    Ok,
    #[cfg_attr(feature = "cli", value(name = "outage"))]
    #[serde(rename = "outage")]
    Outage,
}
impl std::fmt::Display for GetApiVersionRegionsResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionRegionsResponseDataItemStatusEnum::Ok => "ok",
            GetApiVersionRegionsResponseDataItemStatusEnum::Outage => "outage",
        };
        write!(f, "{}", str_val)
    }
}
