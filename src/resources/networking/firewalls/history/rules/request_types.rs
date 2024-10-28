#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    pub firewall_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    pub version: i64,
}
