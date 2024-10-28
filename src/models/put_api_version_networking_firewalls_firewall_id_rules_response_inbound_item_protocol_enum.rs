#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionNetworkingFirewallsFirewallIdRulesResponseInboundItemProtocolEnum {
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
for PutApiVersionNetworkingFirewallsFirewallIdRulesResponseInboundItemProtocolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionNetworkingFirewallsFirewallIdRulesResponseInboundItemProtocolEnum::Icmp => {
                "ICMP"
            }
            PutApiVersionNetworkingFirewallsFirewallIdRulesResponseInboundItemProtocolEnum::Ipencap => {
                "IPENCAP"
            }
            PutApiVersionNetworkingFirewallsFirewallIdRulesResponseInboundItemProtocolEnum::Tcp => {
                "TCP"
            }
            PutApiVersionNetworkingFirewallsFirewallIdRulesResponseInboundItemProtocolEnum::Udp => {
                "UDP"
            }
        };
        write!(f, "{}", str_val)
    }
}
