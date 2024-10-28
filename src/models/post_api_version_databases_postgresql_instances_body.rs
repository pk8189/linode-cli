#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionDatabasesPostgresqlInstancesBody {
    #[cfg_attr(feature = "cli", arg(id = "allow-list", long = "allow-list"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "cluster-size", long = "cluster-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "encrypted", long = "encrypted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "engine", long = "engine"))]
    pub engine: String,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    pub label: String,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    pub region: String,
    #[cfg_attr(
        feature = "cli",
        arg(id = "replication-commit-type", long = "replication-commit-type")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_commit_type: Option<
        crate::models::PostApiVersionDatabasesPostgresqlInstancesBodyReplicationCommitTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "replication-type", long = "replication-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<
        crate::models::PostApiVersionDatabasesPostgresqlInstancesBodyReplicationTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "ssl-connection", long = "ssl-connection"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_connection: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    pub type_field: String,
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
