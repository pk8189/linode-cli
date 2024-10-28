#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "interface-id", long = "interface-id"))]
    pub interface_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "interface-id", long = "interface-id"))]
    pub interface_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<serde_json::Value>)
    )]
    pub data: serde_json::Value,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    pub linode_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "interface-id", long = "interface-id"))]
    pub interface_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBody,
}
