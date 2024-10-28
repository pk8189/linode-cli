#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "deleted"))]
    #[serde(rename = "deleted")]
    Deleted,
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
    #[cfg_attr(feature = "cli", value(name = "enabled"))]
    #[serde(rename = "enabled")]
    Enabled,
}
impl std::fmt::Display
for GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionResponseStatusEnum::Deleted => {
                "deleted"
            }
            GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionResponseStatusEnum::Disabled => {
                "disabled"
            }
            GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionResponseStatusEnum::Enabled => {
                "enabled"
            }
        };
        write!(f, "{}", str_val)
    }
}
