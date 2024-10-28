#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionNetworkingFirewallsFirewallIdRulesApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    pub firewall_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionNetworkingFirewallsFirewallIdRulesApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    pub firewall_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBody,
}
