#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "domain-id", long = "domain-id"))]
    pub domain_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "record-id", long = "record-id"))]
    pub record_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionDomainsDomainIdRecordsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "domain-id", long = "domain-id"))]
    pub domain_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "page", long = "page"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "page-size", long = "page-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "domain-id", long = "domain-id"))]
    pub domain_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "record-id", long = "record-id"))]
    pub record_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionDomainsDomainIdRecordsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "domain-id", long = "domain-id"))]
    pub domain_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionDomainsDomainIdRecordsBody,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "domain-id", long = "domain-id"))]
    pub domain_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "record-id", long = "record-id"))]
    pub record_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionDomainsDomainIdRecordsRecordIdBody,
}
