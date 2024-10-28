#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionManagedIssuesIssueIdResponseEntityTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ticket"))]
    #[serde(rename = "ticket")]
    Ticket,
}
impl std::fmt::Display for GetApiVersionManagedIssuesIssueIdResponseEntityTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionManagedIssuesIssueIdResponseEntityTypeEnum::Ticket => "ticket",
        };
        write!(f, "{}", str_val)
    }
}
