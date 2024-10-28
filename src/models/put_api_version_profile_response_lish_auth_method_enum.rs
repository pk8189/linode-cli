#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionProfileResponseLishAuthMethodEnum {
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
impl std::fmt::Display for PutApiVersionProfileResponseLishAuthMethodEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionProfileResponseLishAuthMethodEnum::Disabled => "disabled",
            PutApiVersionProfileResponseLishAuthMethodEnum::KeysOnly => "keys_only",
            PutApiVersionProfileResponseLishAuthMethodEnum::PasswordKeys => {
                "password_keys"
            }
        };
        write!(f, "{}", str_val)
    }
}
