#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionAccountUsersUsernameGrantsBody {
    #[cfg_attr(feature = "cli", arg(id = "database", long = "database"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyDatabaseItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyDatabaseItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "domain", long = "domain"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyDomainItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyDomainItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "firewall", long = "firewall"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyFirewallItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyFirewallItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "global", long = "global"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyGlobal>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyGlobal>,
    #[cfg_attr(feature = "cli", arg(id = "image", long = "image"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyImageItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyImageItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "linode", long = "linode"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyLinodeItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyLinodeItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "longview", long = "longview"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyLongviewItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longview: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyLongviewItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "nodebalancer", long = "nodebalancer"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyNodebalancerItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodebalancer: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyNodebalancerItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "placement-group", long = "placement-group"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyPlacementGroupItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyPlacementGroupItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "stackscript", long = "stackscript"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyStackscriptItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackscript: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyStackscriptItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "volume", long = "volume"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyVolumeItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyVolumeItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "vpc", long = "vpc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyVpcItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<
        Vec<crate::models::PutApiVersionAccountUsersUsernameGrantsBodyVpcItem>,
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
