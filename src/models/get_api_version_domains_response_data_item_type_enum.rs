#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionDomainsResponseDataItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "master"))]
    #[serde(rename = "master")]
    Master,
    #[cfg_attr(feature = "cli", value(name = "slave"))]
    #[serde(rename = "slave")]
    Slave,
}
impl std::fmt::Display for GetApiVersionDomainsResponseDataItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionDomainsResponseDataItemTypeEnum::Master => "master",
            GetApiVersionDomainsResponseDataItemTypeEnum::Slave => "slave",
        };
        write!(f, "{}", str_val)
    }
}
