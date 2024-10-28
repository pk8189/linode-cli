#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionVpcsVpcIdApiVersionEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "v4"))]
    #[serde(rename = "v4")]
    V4,
    #[cfg_attr(feature = "cli", value(name = "v4beta"))]
    #[serde(rename = "v4beta")]
    V4beta,
}
impl std::fmt::Display for PutApiVersionVpcsVpcIdApiVersionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionVpcsVpcIdApiVersionEnum::V4 => "v4",
            PutApiVersionVpcsVpcIdApiVersionEnum::V4beta => "v4beta",
        };
        write!(f, "{}", str_val)
    }
}
