#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionDomainsResponseTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "master"))]
    #[serde(rename = "master")]
    Master,
    #[cfg_attr(feature = "cli", value(name = "slave"))]
    #[serde(rename = "slave")]
    Slave,
}
impl std::fmt::Display for PostApiVersionDomainsResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionDomainsResponseTypeEnum::Master => "master",
            PostApiVersionDomainsResponseTypeEnum::Slave => "slave",
        };
        write!(f, "{}", str_val)
    }
}
