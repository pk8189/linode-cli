#[derive(Clone, Debug)]
pub struct NodesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl NodesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn recycle(
        &self,
    ) -> crate::resources::lke::clusters::nodes::recycle::resource_client::RecycleClient {
        crate::resources::lke::clusters::nodes::recycle::resource_client::RecycleClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a specific Node from a Node Pool.
    ///
    /// __Deleting a Node is a destructive action and cannot be undone.__
    ///
    /// Deleting a Node will reduce the size of the Node Pool it belongs to.
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
    ///     linode-cli lke node-delete 12345 12345-6aa78910bc
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     lke:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn delete(
        &self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}/nodes/{}", & request.api_version, & request
                    .cluster_id, & request.node_id
                ),
            );
        let mut builder = reqwest::Client::default().delete(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
    /// Returns the values for a specified node object.
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
    ///     linode-cli lke node-view 123456 12345-6aa78910bc
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     lke:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLkeClustersClusterIdNodesNodeIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}/nodes/{}", & request.api_version, & request
                    .cluster_id, & request.node_id
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
            crate::models::GetApiVersionLkeClustersClusterIdNodesNodeIdResponse,
        >(response)
            .await
    }
}
