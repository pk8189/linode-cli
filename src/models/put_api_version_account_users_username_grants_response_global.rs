#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionAccountUsersUsernameGrantsResponseGlobal {
    #[cfg_attr(feature = "cli", arg(id = "account-access", long = "account-access"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<crate::models::PutApiVersionAccountUsersUsernameGrantsResponseGlobalAccountAccessEnum>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub account_access: crate::core::patch::Patch<
        crate::models::PutApiVersionAccountUsersUsernameGrantsResponseGlobalAccountAccessEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "add-databases", long = "add-databases"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_databases: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-domains", long = "add-domains"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_domains: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-firewalls", long = "add-firewalls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_firewalls: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-images", long = "add-images"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_images: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-linodes", long = "add-linodes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_linodes: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "add-loadbalancers", long = "add-loadbalancers")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_loadbalancers: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-longview", long = "add-longview"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_longview: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "add-nodebalancers", long = "add-nodebalancers")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_nodebalancers: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "add-placement-groups", long = "add-placement-groups")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_placement_groups: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-stackscripts", long = "add-stackscripts"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_stackscripts: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-volumes", long = "add-volumes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_volumes: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "add-vpcs", long = "add-vpcs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_vpcs: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "cancel-account", long = "cancel-account"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_account: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "child-account-access", long = "child-account-access")
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<bool>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub child_account_access: crate::core::patch::Patch<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "longview-subscription", long = "longview-subscription")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longview_subscription: Option<bool>,
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
