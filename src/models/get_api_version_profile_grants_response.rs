#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionProfileGrantsResponse {
    #[cfg_attr(feature = "cli", arg(id = "database", long = "database"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseDatabaseItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<
        Vec<crate::models::GetApiVersionProfileGrantsResponseDatabaseItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "domain", long = "domain"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseDomainItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<crate::models::GetApiVersionProfileGrantsResponseDomainItem>>,
    #[cfg_attr(feature = "cli", arg(id = "firewall", long = "firewall"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseFirewallItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<
        Vec<crate::models::GetApiVersionProfileGrantsResponseFirewallItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "global", long = "global"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseGlobal>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<crate::models::GetApiVersionProfileGrantsResponseGlobal>,
    #[cfg_attr(feature = "cli", arg(id = "image", long = "image"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseImageItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<crate::models::GetApiVersionProfileGrantsResponseImageItem>>,
    #[cfg_attr(feature = "cli", arg(id = "linode", long = "linode"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseLinodeItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode: Option<Vec<crate::models::GetApiVersionProfileGrantsResponseLinodeItem>>,
    #[cfg_attr(feature = "cli", arg(id = "longview", long = "longview"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseLongviewItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longview: Option<
        Vec<crate::models::GetApiVersionProfileGrantsResponseLongviewItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "nodebalancer", long = "nodebalancer"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseNodebalancerItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodebalancer: Option<
        Vec<crate::models::GetApiVersionProfileGrantsResponseNodebalancerItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "placement-group", long = "placement-group"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponsePlacementGroupItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<
        Vec<crate::models::GetApiVersionProfileGrantsResponsePlacementGroupItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "stackscript", long = "stackscript"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseStackscriptItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackscript: Option<
        Vec<crate::models::GetApiVersionProfileGrantsResponseStackscriptItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "volume", long = "volume"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseVolumeItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<crate::models::GetApiVersionProfileGrantsResponseVolumeItem>>,
    #[cfg_attr(feature = "cli", arg(id = "vpc", long = "vpc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionProfileGrantsResponseVpcItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<Vec<crate::models::GetApiVersionProfileGrantsResponseVpcItem>>,
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
