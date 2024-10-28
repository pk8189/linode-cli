#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountEventsEventIdResponseEntityTypeEnum {
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
impl std::fmt::Display for GetApiVersionAccountEventsEventIdResponseEntityTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Account => "account",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Backups => "backups",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Community => {
                "community"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Disks => "disks",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Domain => "domain",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::EntityTransfer => {
                "entity_transfer"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Firewall => {
                "firewall"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Image => "image",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Ipaddress => {
                "ipaddress"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Linode => "linode",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Loadbalancer => {
                "loadbalancer"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Longview => {
                "longview"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::ManagedService => {
                "managed_service"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Nodebalancer => {
                "nodebalancer"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::OauthClient => {
                "oauth_client"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Profile => "profile",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Stackscript => {
                "stackscript"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Tag => "tag",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Ticket => "ticket",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Token => "token",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::User => "user",
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::UserSshKey => {
                "user_ssh_key"
            }
            GetApiVersionAccountEventsEventIdResponseEntityTypeEnum::Volume => "volume",
        };
        write!(f, "{}", str_val)
    }
}
