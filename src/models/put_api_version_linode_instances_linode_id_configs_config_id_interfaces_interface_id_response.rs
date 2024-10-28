#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "active", long = "active"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "ip-ranges", long = "ip-ranges"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<Vec<String>>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub ip_ranges: crate::core::patch::Patch<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "ipam-address", long = "ipam-address"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub ipam_address: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "ipv4", long = "ipv4"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponseIpv4>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponseIpv4,
    >,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub label: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "primary", long = "primary"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "purpose", long = "purpose"))]
    pub purpose: crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum,
    #[cfg_attr(feature = "cli", arg(id = "subnet-id", long = "subnet-id"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub subnet_id: crate::core::patch::Patch<i64>,
    #[cfg_attr(feature = "cli", arg(id = "vpc-id", long = "vpc-id"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub vpc_id: crate::core::patch::Patch<i64>,
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
