#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionAccountUsersUsernameGrantsResponse {
    #[cfg_attr(feature = "cli", arg(id = "database", long = "database"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseDatabaseItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseDatabaseItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "domain", long = "domain"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseDomainItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseDomainItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "firewall", long = "firewall"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseFirewallItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseFirewallItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "global", long = "global"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseGlobal>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<
        crate::models::PutApiVersionAccountUsersUsernameGrantsResponseGlobal,
    >,
    #[cfg_attr(feature = "cli", arg(id = "image", long = "image"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseImageItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseImageItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "linode", long = "linode"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseLinodeItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseLinodeItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "longview", long = "longview"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseLongviewItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longview: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseLongviewItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "nodebalancer", long = "nodebalancer"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseNodebalancerItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodebalancer: Option<
        Vec<
            crate::models::PutApiVersionAccountUsersUsernameGrantsResponseNodebalancerItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "placement-group", long = "placement-group"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponsePlacementGroupItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<
        Vec<
            crate::models::PutApiVersionAccountUsersUsernameGrantsResponsePlacementGroupItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "stackscript", long = "stackscript"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseStackscriptItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackscript: Option<
        Vec<
            crate::models::PutApiVersionAccountUsersUsernameGrantsResponseStackscriptItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "volume", long = "volume"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseVolumeItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseVolumeItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "vpc", long = "vpc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseVpcItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseVpcItem>,
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
