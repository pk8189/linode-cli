#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdIpsAddressResponseVpcNat11 {
    #[cfg_attr(feature = "cli", arg(id = "address", long = "address"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "subnet-id", long = "subnet-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<i64>,
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
