#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountOauthClientsResponseDataItemStatusEnum {
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
impl std::fmt::Display for GetApiVersionAccountOauthClientsResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountOauthClientsResponseDataItemStatusEnum::Active => {
                "active"
            }
            GetApiVersionAccountOauthClientsResponseDataItemStatusEnum::Disabled => {
                "disabled"
            }
            GetApiVersionAccountOauthClientsResponseDataItemStatusEnum::Suspended => {
                "suspended"
            }
        };
        write!(f, "{}", str_val)
    }
}
