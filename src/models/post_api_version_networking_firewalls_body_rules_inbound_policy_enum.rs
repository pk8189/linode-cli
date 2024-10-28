#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNetworkingFirewallsBodyRulesInboundPolicyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ACCEPT"))]
    #[serde(rename = "ACCEPT")]
    Accept,
    #[cfg_attr(feature = "cli", value(name = "DROP"))]
    #[serde(rename = "DROP")]
    Drop,
}
impl std::fmt::Display for PostApiVersionNetworkingFirewallsBodyRulesInboundPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNetworkingFirewallsBodyRulesInboundPolicyEnum::Accept => {
                "ACCEPT"
            }
            PostApiVersionNetworkingFirewallsBodyRulesInboundPolicyEnum::Drop => "DROP",
        };
        write!(f, "{}", str_val)
    }
}
