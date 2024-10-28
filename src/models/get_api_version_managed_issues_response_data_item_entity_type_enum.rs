#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionManagedIssuesResponseDataItemEntityTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ticket"))]
    #[serde(rename = "ticket")]
    Ticket,
}
impl std::fmt::Display for GetApiVersionManagedIssuesResponseDataItemEntityTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionManagedIssuesResponseDataItemEntityTypeEnum::Ticket => "ticket",
        };
        write!(f, "{}", str_val)
    }
}
