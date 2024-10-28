#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBody {
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    pub label: String,
    #[cfg_attr(feature = "cli", arg(id = "target", long = "target"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<
        crate::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBodyTargetEnum,
    >,
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
