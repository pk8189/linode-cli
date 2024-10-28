#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionDomainsDomainIdRecordsResponseTagEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "iodef"))]
    #[serde(rename = "iodef")]
    Iodef,
    #[cfg_attr(feature = "cli", value(name = "issue"))]
    #[serde(rename = "issue")]
    Issue,
    #[cfg_attr(feature = "cli", value(name = "issuewild"))]
    #[serde(rename = "issuewild")]
    Issuewild,
}
impl std::fmt::Display for PostApiVersionDomainsDomainIdRecordsResponseTagEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionDomainsDomainIdRecordsResponseTagEnum::Iodef => "iodef",
            PostApiVersionDomainsDomainIdRecordsResponseTagEnum::Issue => "issue",
            PostApiVersionDomainsDomainIdRecordsResponseTagEnum::Issuewild => "issuewild",
        };
        write!(f, "{}", str_val)
    }
}
