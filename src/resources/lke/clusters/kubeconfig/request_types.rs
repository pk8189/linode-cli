#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionLkeClustersClusterIdKubeconfigApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "cluster-id", long = "cluster-id"))]
    pub cluster_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionLkeClustersClusterIdKubeconfigApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "cluster-id", long = "cluster-id"))]
    pub cluster_id: i64,
}
