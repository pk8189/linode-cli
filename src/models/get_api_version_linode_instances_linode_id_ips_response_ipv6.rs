#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6 {
    #[cfg_attr(feature = "cli", arg(id = "global", long = "global"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6GlobalItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<
        Vec<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6GlobalItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "link-local", long = "link-local"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6LinkLocal>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6LinkLocal,
    >,
    #[cfg_attr(feature = "cli", arg(id = "slaac", long = "slaac"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6Slaac>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slaac: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv6Slaac,
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
