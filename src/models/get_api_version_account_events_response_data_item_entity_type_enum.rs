#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountEventsResponseDataItemEntityTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "account"))]
    #[serde(rename = "account")]
    Account,
    #[cfg_attr(feature = "cli", value(name = "backups"))]
    #[serde(rename = "backups")]
    Backups,
    #[cfg_attr(feature = "cli", value(name = "community"))]
    #[serde(rename = "community")]
    Community,
    #[cfg_attr(feature = "cli", value(name = "disks"))]
    #[serde(rename = "disks")]
    Disks,
    #[cfg_attr(feature = "cli", value(name = "domain"))]
    #[serde(rename = "domain")]
    Domain,
    #[cfg_attr(feature = "cli", value(name = "entity_transfer"))]
    #[serde(rename = "entity_transfer")]
    EntityTransfer,
    #[cfg_attr(feature = "cli", value(name = "firewall"))]
    #[serde(rename = "firewall")]
    Firewall,
    #[cfg_attr(feature = "cli", value(name = "image"))]
    #[serde(rename = "image")]
    Image,
    #[cfg_attr(feature = "cli", value(name = "ipaddress"))]
    #[serde(rename = "ipaddress")]
    Ipaddress,
    #[cfg_attr(feature = "cli", value(name = "linode"))]
    #[serde(rename = "linode")]
    Linode,
    #[cfg_attr(feature = "cli", value(name = "loadbalancer"))]
    #[serde(rename = "loadbalancer")]
    Loadbalancer,
    #[cfg_attr(feature = "cli", value(name = "longview"))]
    #[serde(rename = "longview")]
    Longview,
    #[cfg_attr(feature = "cli", value(name = "managed_service"))]
    #[serde(rename = "managed_service")]
    ManagedService,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer"))]
    #[serde(rename = "nodebalancer")]
    Nodebalancer,
    #[cfg_attr(feature = "cli", value(name = "oauth_client"))]
    #[serde(rename = "oauth_client")]
    OauthClient,
    #[cfg_attr(feature = "cli", value(name = "profile"))]
    #[serde(rename = "profile")]
    Profile,
    #[cfg_attr(feature = "cli", value(name = "stackscript"))]
    #[serde(rename = "stackscript")]
    Stackscript,
    #[cfg_attr(feature = "cli", value(name = "tag"))]
    #[serde(rename = "tag")]
    Tag,
    #[cfg_attr(feature = "cli", value(name = "ticket"))]
    #[serde(rename = "ticket")]
    Ticket,
    #[cfg_attr(feature = "cli", value(name = "token"))]
    #[serde(rename = "token")]
    Token,
    #[cfg_attr(feature = "cli", value(name = "user"))]
    #[serde(rename = "user")]
    User,
    #[cfg_attr(feature = "cli", value(name = "user_ssh_key"))]
    #[serde(rename = "user_ssh_key")]
    UserSshKey,
    #[cfg_attr(feature = "cli", value(name = "volume"))]
    #[serde(rename = "volume")]
    Volume,
}
impl std::fmt::Display for GetApiVersionAccountEventsResponseDataItemEntityTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Account => {
                "account"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Backups => {
                "backups"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Community => {
                "community"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Disks => "disks",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Domain => "domain",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::EntityTransfer => {
                "entity_transfer"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Firewall => {
                "firewall"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Image => "image",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Ipaddress => {
                "ipaddress"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Linode => "linode",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Loadbalancer => {
                "loadbalancer"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Longview => {
                "longview"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::ManagedService => {
                "managed_service"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Nodebalancer => {
                "nodebalancer"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::OauthClient => {
                "oauth_client"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Profile => {
                "profile"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Stackscript => {
                "stackscript"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Tag => "tag",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Ticket => "ticket",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Token => "token",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::User => "user",
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::UserSshKey => {
                "user_ssh_key"
            }
            GetApiVersionAccountEventsResponseDataItemEntityTypeEnum::Volume => "volume",
        };
        write!(f, "{}", str_val)
    }
}
