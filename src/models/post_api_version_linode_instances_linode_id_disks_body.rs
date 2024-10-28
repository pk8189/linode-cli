#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdDisksBody {
    #[cfg_attr(feature = "cli", arg(id = "authorized-keys", long = "authorized-keys"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_keys: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "authorized-users", long = "authorized-users"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_users: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "filesystem", long = "filesystem"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdDisksBodyFilesystemEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "image", long = "image"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "root-pass", long = "root-pass"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_pass: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "size", long = "size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "stackscript-data", long = "stackscript-data"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<serde_json::Value>)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackscript_data: Option<serde_json::Value>,
    #[cfg_attr(feature = "cli", arg(id = "stackscript-id", long = "stackscript-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackscript_id: Option<i64>,
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
