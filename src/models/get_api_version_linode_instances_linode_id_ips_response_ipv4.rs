#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4 {
    #[cfg_attr(feature = "cli", arg(id = "private", long = "private"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4PrivateItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<
        Vec<
            crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4PrivateItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "public", long = "public"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4PublicItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<
        Vec<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4PublicItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "reserved", long = "reserved"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<
        Vec<
            crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "shared", long = "shared"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4SharedItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<
        Vec<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4SharedItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "vpc", long = "vpc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4VpcItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<
        Vec<crate::models::GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4VpcItem>,
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
