#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionProfileBodyLishAuthMethodEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
    #[cfg_attr(feature = "cli", value(name = "keys_only"))]
    #[serde(rename = "keys_only")]
    KeysOnly,
    #[cfg_attr(feature = "cli", value(name = "password_keys"))]
    #[serde(rename = "password_keys")]
    PasswordKeys,
}
impl std::fmt::Display for PutApiVersionProfileBodyLishAuthMethodEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionProfileBodyLishAuthMethodEnum::Disabled => "disabled",
            PutApiVersionProfileBodyLishAuthMethodEnum::KeysOnly => "keys_only",
            PutApiVersionProfileBodyLishAuthMethodEnum::PasswordKeys => "password_keys",
        };
        write!(f, "{}", str_val)
    }
}
