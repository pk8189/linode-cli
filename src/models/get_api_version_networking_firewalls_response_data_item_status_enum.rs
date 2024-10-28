#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNetworkingFirewallsResponseDataItemStatusEnum {
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
impl std::fmt::Display for GetApiVersionNetworkingFirewallsResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNetworkingFirewallsResponseDataItemStatusEnum::Deleted => {
                "deleted"
            }
            GetApiVersionNetworkingFirewallsResponseDataItemStatusEnum::Disabled => {
                "disabled"
            }
            GetApiVersionNetworkingFirewallsResponseDataItemStatusEnum::Enabled => {
                "enabled"
            }
        };
        write!(f, "{}", str_val)
    }
}
