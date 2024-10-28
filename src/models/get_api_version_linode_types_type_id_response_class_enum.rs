#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeTypesTypeIdResponseClassEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "dedicated"))]
    #[serde(rename = "dedicated")]
    Dedicated,
    #[cfg_attr(feature = "cli", value(name = "gpu"))]
    #[serde(rename = "gpu")]
    Gpu,
    #[cfg_attr(feature = "cli", value(name = "highmem"))]
    #[serde(rename = "highmem")]
    Highmem,
    #[cfg_attr(feature = "cli", value(name = "nanode"))]
    #[serde(rename = "nanode")]
    Nanode,
    #[cfg_attr(feature = "cli", value(name = "premium"))]
    #[serde(rename = "premium")]
    Premium,
    #[cfg_attr(feature = "cli", value(name = "standard"))]
    #[serde(rename = "standard")]
    Standard,
}
impl std::fmt::Display for GetApiVersionLinodeTypesTypeIdResponseClassEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeTypesTypeIdResponseClassEnum::Dedicated => "dedicated",
            GetApiVersionLinodeTypesTypeIdResponseClassEnum::Gpu => "gpu",
            GetApiVersionLinodeTypesTypeIdResponseClassEnum::Highmem => "highmem",
            GetApiVersionLinodeTypesTypeIdResponseClassEnum::Nanode => "nanode",
            GetApiVersionLinodeTypesTypeIdResponseClassEnum::Premium => "premium",
            GetApiVersionLinodeTypesTypeIdResponseClassEnum::Standard => "standard",
        };
        write!(f, "{}", str_val)
    }
}
