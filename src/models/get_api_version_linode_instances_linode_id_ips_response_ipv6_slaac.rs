#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6Slaac {
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "gateway", long = "gateway"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "prefix", long = "prefix"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "public", long = "public"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "rdns", long = "rdns"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdns: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "subnet-mask", long = "subnet-mask"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
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
