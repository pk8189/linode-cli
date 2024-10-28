#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBody {
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
    #[cfg_attr(feature = "cli", arg(id = "ipv4", long = "ipv4"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBodyIpv4>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBodyIpv4,
    >,
    #[cfg_attr(feature = "cli", arg(id = "primary", long = "primary"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
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
