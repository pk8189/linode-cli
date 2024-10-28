#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionDomainsDomainIdRecordsRecordIdResponseTagEnum {
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
impl std::fmt::Display for GetApiVersionDomainsDomainIdRecordsRecordIdResponseTagEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTagEnum::Iodef => "iodef",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTagEnum::Issue => "issue",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTagEnum::Issuewild => {
                "issuewild"
            }
        };
        write!(f, "{}", str_val)
    }
}
