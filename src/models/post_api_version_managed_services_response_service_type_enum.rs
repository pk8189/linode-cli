#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionManagedServicesResponseServiceTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "tcp"))]
    #[serde(rename = "tcp")]
    Tcp,
    #[cfg_attr(feature = "cli", value(name = "url"))]
    #[serde(rename = "url")]
    Url,
}
impl std::fmt::Display for PostApiVersionManagedServicesResponseServiceTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionManagedServicesResponseServiceTypeEnum::Tcp => "tcp",
            PostApiVersionManagedServicesResponseServiceTypeEnum::Url => "url",
        };
        write!(f, "{}", str_val)
    }
}
