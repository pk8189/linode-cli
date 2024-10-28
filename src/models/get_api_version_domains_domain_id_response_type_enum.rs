#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionDomainsDomainIdResponseTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "master"))]
    #[serde(rename = "master")]
    Master,
    #[cfg_attr(feature = "cli", value(name = "slave"))]
    #[serde(rename = "slave")]
    Slave,
}
impl std::fmt::Display for GetApiVersionDomainsDomainIdResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionDomainsDomainIdResponseTypeEnum::Master => "master",
            GetApiVersionDomainsDomainIdResponseTypeEnum::Slave => "slave",
        };
        write!(f, "{}", str_val)
    }
}
