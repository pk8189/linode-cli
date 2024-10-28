#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNetworkingFirewallsResponseRulesInboundPolicyEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ACCEPT"))]
    #[serde(rename = "ACCEPT")]
    Accept,
    #[cfg_attr(feature = "cli", value(name = "DROP"))]
    #[serde(rename = "DROP")]
    Drop,
}
impl std::fmt::Display
for PostApiVersionNetworkingFirewallsResponseRulesInboundPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNetworkingFirewallsResponseRulesInboundPolicyEnum::Accept => {
                "ACCEPT"
            }
            PostApiVersionNetworkingFirewallsResponseRulesInboundPolicyEnum::Drop => {
                "DROP"
            }
        };
        write!(f, "{}", str_val)
    }
}
