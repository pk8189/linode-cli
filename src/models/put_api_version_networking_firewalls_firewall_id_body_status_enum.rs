#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionNetworkingFirewallsFirewallIdBodyStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
    #[cfg_attr(feature = "cli", value(name = "enabled"))]
    #[serde(rename = "enabled")]
    Enabled,
}
impl std::fmt::Display for PutApiVersionNetworkingFirewallsFirewallIdBodyStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionNetworkingFirewallsFirewallIdBodyStatusEnum::Disabled => {
                "disabled"
            }
            PutApiVersionNetworkingFirewallsFirewallIdBodyStatusEnum::Enabled => {
                "enabled"
            }
        };
        write!(f, "{}", str_val)
    }
}
