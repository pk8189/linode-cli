#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountNotificationsResponseDataItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "maintenance"))]
    #[serde(rename = "maintenance")]
    Maintenance,
    #[cfg_attr(feature = "cli", value(name = "migration_imminent"))]
    #[serde(rename = "migration_imminent")]
    MigrationImminent,
    #[cfg_attr(feature = "cli", value(name = "migration_pending"))]
    #[serde(rename = "migration_pending")]
    MigrationPending,
    #[cfg_attr(feature = "cli", value(name = "migration_scheduled"))]
    #[serde(rename = "migration_scheduled")]
    MigrationScheduled,
    #[cfg_attr(feature = "cli", value(name = "notice"))]
    #[serde(rename = "notice")]
    Notice,
    #[cfg_attr(feature = "cli", value(name = "outage"))]
    #[serde(rename = "outage")]
    Outage,
    #[cfg_attr(feature = "cli", value(name = "payment_due"))]
    #[serde(rename = "payment_due")]
    PaymentDue,
    #[cfg_attr(feature = "cli", value(name = "promotion"))]
    #[serde(rename = "promotion")]
    Promotion,
    #[cfg_attr(feature = "cli", value(name = "reboot_scheduled"))]
    #[serde(rename = "reboot_scheduled")]
    RebootScheduled,
    #[cfg_attr(feature = "cli", value(name = "ticket_abuse"))]
    #[serde(rename = "ticket_abuse")]
    TicketAbuse,
    #[cfg_attr(feature = "cli", value(name = "ticket_important"))]
    #[serde(rename = "ticket_important")]
    TicketImportant,
}
impl std::fmt::Display for GetApiVersionAccountNotificationsResponseDataItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::Maintenance => {
                "maintenance"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::MigrationImminent => {
                "migration_imminent"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::MigrationPending => {
                "migration_pending"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::MigrationScheduled => {
                "migration_scheduled"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::Notice => "notice",
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::Outage => "outage",
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::PaymentDue => {
                "payment_due"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::Promotion => {
                "promotion"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::RebootScheduled => {
                "reboot_scheduled"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::TicketAbuse => {
                "ticket_abuse"
            }
            GetApiVersionAccountNotificationsResponseDataItemTypeEnum::TicketImportant => {
                "ticket_important"
            }
        };
        write!(f, "{}", str_val)
    }
}
