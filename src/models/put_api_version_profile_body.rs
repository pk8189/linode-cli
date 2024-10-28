#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionProfileBody {
    #[cfg_attr(
        feature = "cli",
        arg(id = "authentication-type", long = "authentication-type")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<
        crate::models::PutApiVersionProfileBodyAuthenticationTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "authorized-keys", long = "authorized-keys"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<Vec<String>>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub authorized_keys: crate::core::patch::Patch<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "email", long = "email"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "email-notifications", long = "email-notifications")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_notifications: Option<bool>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "ip-whitelist-enabled", long = "ip-whitelist-enabled")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelist_enabled: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "lish-auth-method", long = "lish-auth-method"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lish_auth_method: Option<
        crate::models::PutApiVersionProfileBodyLishAuthMethodEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "referrals", long = "referrals"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionProfileBodyReferrals>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrals: Option<crate::models::PutApiVersionProfileBodyReferrals>,
    #[cfg_attr(feature = "cli", arg(id = "restricted", long = "restricted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "timezone", long = "timezone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "two-factor-auth", long = "two-factor-auth"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_auth: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "uid", long = "uid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
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
