#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionManagedServicesServiceIdResponseServiceTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "tcp"))]
    #[serde(rename = "tcp")]
    Tcp,
    #[cfg_attr(feature = "cli", value(name = "url"))]
    #[serde(rename = "url")]
    Url,
}
impl std::fmt::Display for PutApiVersionManagedServicesServiceIdResponseServiceTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionManagedServicesServiceIdResponseServiceTypeEnum::Tcp => "tcp",
            PutApiVersionManagedServicesServiceIdResponseServiceTypeEnum::Url => "url",
        };
        write!(f, "{}", str_val)
    }
}
