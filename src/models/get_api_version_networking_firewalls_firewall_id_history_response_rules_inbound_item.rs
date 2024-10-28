#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesInboundItem {
    #[cfg_attr(feature = "cli", arg(id = "action", long = "action"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesInboundItemActionEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "addresses", long = "addresses"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesInboundItemAddresses>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesInboundItemAddresses,
    >,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "ports", long = "ports"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub ports: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "protocol", long = "protocol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryResponseRulesInboundItemProtocolEnum,
    >,
    #[serde(flatten)]
    #[cfg_attr(
        feature = "cli",
        arg(
            id = "additional-props",
            long = "additional-props",
            value_parser = crate::core::clap::parse_json::<std::collections::HashMap<String,
            serde_json::Value>>,
            default_value = "{}",
        )
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
