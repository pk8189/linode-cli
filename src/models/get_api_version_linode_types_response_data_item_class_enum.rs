#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeTypesResponseDataItemClassEnum {
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
impl std::fmt::Display for GetApiVersionLinodeTypesResponseDataItemClassEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeTypesResponseDataItemClassEnum::Dedicated => "dedicated",
            GetApiVersionLinodeTypesResponseDataItemClassEnum::Gpu => "gpu",
            GetApiVersionLinodeTypesResponseDataItemClassEnum::Highmem => "highmem",
            GetApiVersionLinodeTypesResponseDataItemClassEnum::Nanode => "nanode",
            GetApiVersionLinodeTypesResponseDataItemClassEnum::Premium => "premium",
            GetApiVersionLinodeTypesResponseDataItemClassEnum::Standard => "standard",
        };
        write!(f, "{}", str_val)
    }
}
