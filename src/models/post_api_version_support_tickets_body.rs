#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionSupportTicketsBody {
    #[cfg_attr(feature = "cli", arg(id = "database-id", long = "database-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    pub description: String,
    #[cfg_attr(feature = "cli", arg(id = "domain-id", long = "domain-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "lkecluster-id", long = "lkecluster-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lkecluster_id: Option<i64>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "longviewclient-id", long = "longviewclient-id")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longviewclient_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "managed-issue", long = "managed-issue"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_issue: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "nodebalancer-id", long = "nodebalancer-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodebalancer_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "summary", long = "summary"))]
    pub summary: String,
    #[cfg_attr(feature = "cli", arg(id = "vlan", long = "vlan"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "volume-id", long = "volume-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "vpc-id", long = "vpc-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<i64>,
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
