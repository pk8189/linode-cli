#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "A"))]
    #[serde(rename = "A")]
    A,
    #[cfg_attr(feature = "cli", value(name = "AAAA"))]
    #[serde(rename = "AAAA")]
    Aaaa,
    #[cfg_attr(feature = "cli", value(name = "CAA"))]
    #[serde(rename = "CAA")]
    Caa,
    #[cfg_attr(feature = "cli", value(name = "CNAME"))]
    #[serde(rename = "CNAME")]
    Cname,
    #[cfg_attr(feature = "cli", value(name = "MX"))]
    #[serde(rename = "MX")]
    Mx,
    #[cfg_attr(feature = "cli", value(name = "NS"))]
    #[serde(rename = "NS")]
    Ns,
    #[cfg_attr(feature = "cli", value(name = "PTR"))]
    #[serde(rename = "PTR")]
    Ptr,
    #[cfg_attr(feature = "cli", value(name = "SRV"))]
    #[serde(rename = "SRV")]
    Srv,
    #[cfg_attr(feature = "cli", value(name = "TXT"))]
    #[serde(rename = "TXT")]
    Txt,
}
impl std::fmt::Display for GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::A => "A",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Aaaa => "AAAA",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Caa => "CAA",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Cname => "CNAME",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Mx => "MX",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Ns => "NS",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Ptr => "PTR",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Srv => "SRV",
            GetApiVersionDomainsDomainIdRecordsRecordIdResponseTypeEnum::Txt => "TXT",
        };
        write!(f, "{}", str_val)
    }
}
