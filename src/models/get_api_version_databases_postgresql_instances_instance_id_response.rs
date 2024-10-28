#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "allow-list", long = "allow-list"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "cluster-size", long = "cluster-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "created", long = "created"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "encrypted", long = "encrypted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "engine", long = "engine"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "hosts", long = "hosts"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseHosts>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseHosts,
    >,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "port", long = "port"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "replication-commit-type", long = "replication-commit-type")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_commit_type: Option<
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationCommitTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "replication-type", long = "replication-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseReplicationTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "ssl-connection", long = "ssl-connection"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_connection: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseStatusEnum,
    >,
    #[cfg_attr(
        feature = "cli",
        arg(id = "total-disk-size-gb", long = "total-disk-size-gb")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_disk_size_gb: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "updated", long = "updated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "updates", long = "updates"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseUpdates>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponseUpdates,
    >,
    #[cfg_attr(
        feature = "cli",
        arg(id = "used-disk-size-gb", long = "used-disk-size-gb")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_disk_size_gb: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
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
