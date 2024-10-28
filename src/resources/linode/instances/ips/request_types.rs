#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    pub address: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionLinodeInstancesLinodeIdIpsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    pub address: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionLinodeInstancesLinodeIdIpsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionLinodeInstancesLinodeIdIpsBody,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    pub address: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionLinodeInstancesLinodeIdIpsAddressBody,
}
