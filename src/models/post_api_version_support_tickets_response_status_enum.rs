#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionSupportTicketsResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "closed"))]
    #[serde(rename = "closed")]
    Closed,
    #[cfg_attr(feature = "cli", value(name = "new"))]
    #[serde(rename = "new")]
    New,
    #[cfg_attr(feature = "cli", value(name = "open"))]
    #[serde(rename = "open")]
    Open,
}
impl std::fmt::Display for PostApiVersionSupportTicketsResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionSupportTicketsResponseStatusEnum::Closed => "closed",
            PostApiVersionSupportTicketsResponseStatusEnum::New => "new",
            PostApiVersionSupportTicketsResponseStatusEnum::Open => "open",
        };
        write!(f, "{}", str_val)
    }
}
