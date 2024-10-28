#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionAccountOauthClientsClientIdResponseStatusEnum {
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
impl std::fmt::Display for PutApiVersionAccountOauthClientsClientIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionAccountOauthClientsClientIdResponseStatusEnum::Active => {
                "active"
            }
            PutApiVersionAccountOauthClientsClientIdResponseStatusEnum::Disabled => {
                "disabled"
            }
            PutApiVersionAccountOauthClientsClientIdResponseStatusEnum::Suspended => {
                "suspended"
            }
        };
        write!(f, "{}", str_val)
    }
}
