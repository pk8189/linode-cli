#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionNodebalancersNodeBalancerIdStatsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "node-balancer-id", long = "node-balancer-id"))]
    pub node_balancer_id: i64,
}
