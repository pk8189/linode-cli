#[derive(Clone, Debug)]
pub struct ClustersClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ClustersClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns a paginated list of available Object Storage legacy clusters.
    ///
    /// > 📘
    /// >
    /// > This displays deprecated `clusterId` values that represent regions used with older versions of the API. It's maintained for backward compatibility. Run [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region), instead.
    ///
    ///
    /// <<LB>>
    ///
    /// ---
    ///
    ///
    /// - __CLI__.
    ///
    ///     `-`-`
    ///     linode-cli object-storage clusters-list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionObjectStorageClustersResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/object-storage/clusters", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionObjectStorageClustersResponse,
        >(response)
            .await
    }
    /// __Deprecated__ Returns a single Object Storage cluster.
    ///
    /// > 📘
    /// >
    /// > This displays deprecated `clusterId` values that represent regions used with older versions of the API. It's maintained for backward compatibility. Run [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region), instead.
    ///
    ///
    /// <<LB>>
    ///
    /// ---
    ///
    ///
    /// - __CLI__.
    ///
    ///     `-`-`
    ///     linode-cli object-storage clusters-view us-east-1
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionObjectStorageClustersClusterIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/clusters/{}", & request.api_version, & request
                    .cluster_id
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionObjectStorageClustersClusterIdResponse,
        >(response)
            .await
    }
}
