#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesOutboundItemProtocolEnum {
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
for GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesOutboundItemProtocolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesOutboundItemProtocolEnum::Icmp => {
                "ICMP"
            }
            GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesOutboundItemProtocolEnum::Ipencap => {
                "IPENCAP"
            }
            GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesOutboundItemProtocolEnum::Tcp => {
                "TCP"
            }
            GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesOutboundItemProtocolEnum::Udp => {
                "UDP"
            }
        };
        write!(f, "{}", str_val)
    }
}
