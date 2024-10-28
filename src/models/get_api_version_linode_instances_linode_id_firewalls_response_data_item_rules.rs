#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRules {
    #[cfg_attr(feature = "cli", arg(id = "fingerprint", long = "fingerprint"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "inbound", long = "inbound"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesInboundItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<
        Vec<
            crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesInboundItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "inbound-policy", long = "inbound-policy"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_policy: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesInboundPolicyEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "outbound", long = "outbound"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<
        Vec<
            crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "outbound-policy", long = "outbound-policy"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_policy: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponseDataItemRulesOutboundPolicyEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
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
