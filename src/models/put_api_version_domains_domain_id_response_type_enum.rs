#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionDomainsDomainIdResponseTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "master"))]
    #[serde(rename = "master")]
    Master,
    #[cfg_attr(feature = "cli", value(name = "slave"))]
    #[serde(rename = "slave")]
    Slave,
}
impl std::fmt::Display for PutApiVersionDomainsDomainIdResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionDomainsDomainIdResponseTypeEnum::Master => "master",
            PutApiVersionDomainsDomainIdResponseTypeEnum::Slave => "slave",
        };
        write!(f, "{}", str_val)
    }
}
