#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionProfileResponseAuthenticationTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "github"))]
    #[serde(rename = "github")]
    Github,
    #[cfg_attr(feature = "cli", value(name = "password"))]
    #[serde(rename = "password")]
    Password,
}
impl std::fmt::Display for GetApiVersionProfileResponseAuthenticationTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionProfileResponseAuthenticationTypeEnum::Github => "github",
            GetApiVersionProfileResponseAuthenticationTypeEnum::Password => "password",
        };
        write!(f, "{}", str_val)
    }
}
