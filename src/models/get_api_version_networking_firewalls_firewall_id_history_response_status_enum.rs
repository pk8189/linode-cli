#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseStatusEnum {
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
for GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseStatusEnum::Deleted => {
                "deleted"
            }
            GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseStatusEnum::Disabled => {
                "disabled"
            }
            GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseStatusEnum::Enabled => {
                "enabled"
            }
        };
        write!(f, "{}", str_val)
    }
}
