#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionDomainsDomainIdCloneResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "active"))]
    #[serde(rename = "active")]
    Active,
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
}
impl std::fmt::Display for PostApiVersionDomainsDomainIdCloneResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionDomainsDomainIdCloneResponseStatusEnum::Active => "active",
            PostApiVersionDomainsDomainIdCloneResponseStatusEnum::Disabled => "disabled",
        };
        write!(f, "{}", str_val)
    }
}
