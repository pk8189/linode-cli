#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionAccountOauthClientsClientIdBodyStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "active"))]
    #[serde(rename = "active")]
    Active,
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
    #[cfg_attr(feature = "cli", value(name = "suspended"))]
    #[serde(rename = "suspended")]
    Suspended,
}
impl std::fmt::Display for PutApiVersionAccountOauthClientsClientIdBodyStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionAccountOauthClientsClientIdBodyStatusEnum::Active => "active",
            PutApiVersionAccountOauthClientsClientIdBodyStatusEnum::Disabled => {
                "disabled"
            }
            PutApiVersionAccountOauthClientsClientIdBodyStatusEnum::Suspended => {
                "suspended"
            }
        };
        write!(f, "{}", str_val)
    }
}
