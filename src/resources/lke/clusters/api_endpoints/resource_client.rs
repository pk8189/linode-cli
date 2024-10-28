#[derive(Clone, Debug)]
pub struct ApiEndpointsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ApiEndpointsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// List the Kubernetes API server endpoints for this cluster. Please note that it often takes 2-5 minutes before the endpoint is ready after first [creating a new cluster](https://techdocs.akamai.com/linode-api/reference/post-lke-cluster).
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
    ///     linode-cli lke api-endpoints-list 12345
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     lke:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLkeClustersClusterIdApiEndpointsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}/api-endpoints", & request.api_version, & request
                    .cluster_id
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionLkeClustersClusterIdApiEndpointsResponse,
        >(response)
            .await
    }
}
