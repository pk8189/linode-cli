#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionLinodeInstancesLinodeIdTransferApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionLinodeInstancesLinodeIdTransferYearMonthApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "year", long = "year"))]
    pub year: i64,
    #[cfg_attr(feature = "cli", arg(id = "month", long = "month"))]
    pub month: i64,
}
