#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4VpcItem {
    #[cfg_attr(feature = "cli", arg(id = "active", long = "active"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub address: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "address-range", long = "address-range"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub address_range: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "config-id", long = "config-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "gateway", long = "gateway"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub gateway: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "interface-id", long = "interface-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "linode-id", long = "linode-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "nat-1-1", long = "nat-1-1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_1_1: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "prefix", long = "prefix"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub prefix: crate::core::patch::Patch<i64>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "subnet-id", long = "subnet-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "subnet-mask", long = "subnet-mask"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
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
