#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "account"))]
    #[serde(rename = "account")]
    Account,
    #[cfg_attr(feature = "cli", value(name = "entity_transfer"))]
    #[serde(rename = "entity_transfer")]
    EntityTransfer,
    #[cfg_attr(feature = "cli", value(name = "linode"))]
    #[serde(rename = "linode")]
    Linode,
    #[cfg_attr(feature = "cli", value(name = "loadbalancers"))]
    #[serde(rename = "loadbalancers")]
    Loadbalancers,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer"))]
    #[serde(rename = "nodebalancer")]
    Nodebalancer,
    #[cfg_attr(feature = "cli", value(name = "promotion"))]
    #[serde(rename = "promotion")]
    Promotion,
    #[cfg_attr(feature = "cli", value(name = "region"))]
    #[serde(rename = "region")]
    Region,
    #[cfg_attr(feature = "cli", value(name = "ticket"))]
    #[serde(rename = "ticket")]
    Ticket,
    #[cfg_attr(feature = "cli", value(name = "volume"))]
    #[serde(rename = "volume")]
    Volume,
}
impl std::fmt::Display
for GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Account => {
                "account"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::EntityTransfer => {
                "entity_transfer"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Linode => {
                "linode"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Loadbalancers => {
                "loadbalancers"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Nodebalancer => {
                "nodebalancer"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Promotion => {
                "promotion"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Region => {
                "region"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Ticket => {
                "ticket"
            }
            GetApiVersionAccountNotificationsResponseDataItemEntityTypeEnum::Volume => {
                "volume"
            }
        };
        write!(f, "{}", str_val)
    }
}
