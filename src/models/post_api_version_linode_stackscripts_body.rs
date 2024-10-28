#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeStackscriptsBody {
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "deployments-active", long = "deployments-active")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments_active: Option<i64>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "deployments-total", long = "deployments-total")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments_total: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "images", long = "images"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "is-public", long = "is-public"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "mine", long = "mine"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mine: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "rev-note", long = "rev-note"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev_note: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "script", long = "script"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "user-defined-fields", long = "user-defined-fields")
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeStackscriptsBodyUserDefinedFieldsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<
        Vec<crate::models::PostApiVersionLinodeStackscriptsBodyUserDefinedFieldsItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "user-gravatar-id", long = "user-gravatar-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_gravatar_id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "username", long = "username"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
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
