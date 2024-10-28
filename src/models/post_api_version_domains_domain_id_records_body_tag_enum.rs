#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionDomainsDomainIdRecordsBodyTagEnum {
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
impl std::fmt::Display for PostApiVersionDomainsDomainIdRecordsBodyTagEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionDomainsDomainIdRecordsBodyTagEnum::Iodef => "iodef",
            PostApiVersionDomainsDomainIdRecordsBodyTagEnum::Issue => "issue",
            PostApiVersionDomainsDomainIdRecordsBodyTagEnum::Issuewild => "issuewild",
        };
        write!(f, "{}", str_val)
    }
}
