#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionAccountUsersUsernameBody {
    #[cfg_attr(feature = "cli", arg(id = "email", long = "email"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "last-login", long = "last-login"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionAccountUsersUsernameBodyLastLogin>
        )
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<crate::models::PutApiVersionAccountUsersUsernameBodyLastLogin>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub last_login: crate::core::patch::Patch<
        crate::models::PutApiVersionAccountUsersUsernameBodyLastLogin,
    >,
    #[cfg_attr(feature = "cli", arg(id = "password-created", long = "password-created"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub password_created: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "restricted", long = "restricted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "ssh-keys", long = "ssh-keys"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "tfa-enabled", long = "tfa-enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "username", long = "username"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "verified-phone-number", long = "verified-phone-number")
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub verified_phone_number: crate::core::patch::Patch<String>,
    #[serde(flatten)]
    #[cfg_attr(
        feature = "cli",
        arg(
            id = "additional-props",
            long = "additional-props",
            value_parser = crate::core::clap::parse_json::<std::collections::HashMap<String,
            serde_json::Value>>,
            default_value = "{}",
        )
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
