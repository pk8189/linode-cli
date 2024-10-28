#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionLkeClustersClusterIdRegenerateApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "cluster-id", long = "cluster-id"))]
    pub cluster_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionLkeClustersClusterIdRegenerateBody,
}
