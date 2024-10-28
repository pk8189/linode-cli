#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionDomainsResponseDataItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "active"))]
    #[serde(rename = "active")]
    Active,
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
}
impl std::fmt::Display for GetApiVersionDomainsResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionDomainsResponseDataItemStatusEnum::Active => "active",
            GetApiVersionDomainsResponseDataItemStatusEnum::Disabled => "disabled",
        };
        write!(f, "{}", str_val)
    }
}
