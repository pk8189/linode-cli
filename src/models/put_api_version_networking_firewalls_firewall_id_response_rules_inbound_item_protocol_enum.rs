#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionNetworkingFirewallsFirewallIdResponseRulesInboundItemProtocolEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ICMP"))]
    #[serde(rename = "ICMP")]
    Icmp,
    #[cfg_attr(feature = "cli", value(name = "IPENCAP"))]
    #[serde(rename = "IPENCAP")]
    Ipencap,
    #[cfg_attr(feature = "cli", value(name = "TCP"))]
    #[serde(rename = "TCP")]
    Tcp,
    #[cfg_attr(feature = "cli", value(name = "UDP"))]
    #[serde(rename = "UDP")]
    Udp,
}
impl std::fmt::Display
for PutApiVersionNetworkingFirewallsFirewallIdResponseRulesInboundItemProtocolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionNetworkingFirewallsFirewallIdResponseRulesInboundItemProtocolEnum::Icmp => {
                "ICMP"
            }
            PutApiVersionNetworkingFirewallsFirewallIdResponseRulesInboundItemProtocolEnum::Ipencap => {
                "IPENCAP"
            }
            PutApiVersionNetworkingFirewallsFirewallIdResponseRulesInboundItemProtocolEnum::Tcp => {
                "TCP"
            }
            PutApiVersionNetworkingFirewallsFirewallIdResponseRulesInboundItemProtocolEnum::Udp => {
                "UDP"
            }
        };
        write!(f, "{}", str_val)
    }
}
