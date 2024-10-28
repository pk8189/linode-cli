#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "node-balancer-id", long = "node-balancer-id"))]
    pub node_balancer_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    pub config_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<serde_json::Value>)
    )]
    pub data: serde_json::Value,
}
