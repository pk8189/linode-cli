#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNetworkingIpsBodyTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ipv4"))]
    #[serde(rename = "ipv4")]
    Ipv4,
}
impl std::fmt::Display for PostApiVersionNetworkingIpsBodyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNetworkingIpsBodyTypeEnum::Ipv4 => "ipv4",
        };
        write!(f, "{}", str_val)
    }
}
