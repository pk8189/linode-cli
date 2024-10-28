#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItemProtocolEnum {
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
for GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItemProtocolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItemProtocolEnum::Icmp => {
                "ICMP"
            }
            GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItemProtocolEnum::Ipencap => {
                "IPENCAP"
            }
            GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItemProtocolEnum::Tcp => {
                "TCP"
            }
            GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItemProtocolEnum::Udp => {
                "UDP"
            }
        };
        write!(f, "{}", str_val)
    }
}
