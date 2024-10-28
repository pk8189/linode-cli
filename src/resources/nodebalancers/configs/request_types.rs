#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "node-balancer-id", long = "node-balancer-id"))]
    pub node_balancer_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "node-balancer-id", long = "node-balancer-id"))]
    pub node_balancer_id: i64,
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
    pub api_version: crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "node-balancer-id", long = "node-balancer-id"))]
    pub node_balancer_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "node-balancer-id", long = "node-balancer-id"))]
    pub node_balancer_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBody,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "node-balancer-id", long = "node-balancer-id"))]
    pub node_balancer_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBody,
}
