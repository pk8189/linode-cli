#[derive(Clone, Debug)]
pub struct NodesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl NodesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Deletes a Node from this Config. This backend will no longer receive traffic for the configured port of this NodeBalancer.
    ///
    /// This does not change or remove the Linode whose address was used in the creation of this Node.
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
    ///     linode-cli nodebalancers node-delete \
    ///   12345 4567 54321
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     nodebalancers:read_write
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
                    "/{}/nodebalancers/{}/configs/{}/nodes/{}", & request.api_version, &
                    request.node_balancer_id, & request.config_id, & request.node_id
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
    /// Returns a paginated list of NodeBalancer nodes associated with this Config. These are the backends that will be sent traffic for this port.
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
    ///     linode-cli nodebalancers nodes-list 12345 4567
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     nodebalancers:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs/{}/nodes", & request.api_version, &
                    request.node_balancer_id, & request.config_id
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("page", &request.page, false);
        queries.add_option("page_size", &request.page_size, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponse,
        >(response)
            .await
    }
    /// Returns information about a single Node, a backend for this NodeBalancer's configured port.
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
    ///     linode-cli nodebalancers node-view 12345 4567 54321
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     nodebalancers:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs/{}/nodes/{}", & request.api_version, &
                    request.node_balancer_id, & request.config_id, & request.node_id
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
            crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdResponse,
        >(response)
            .await
    }
    /// Creates a NodeBalancer Node, a backend that can accept traffic for this NodeBalancer Config. Nodes are routed requests on the configured port based on their status.
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
    ///     linode-cli nodebalancers node-create \
    ///   12345 4567 \
    ///   --address 192.168.210.120:80 \
    ///   --label node54321 \
    ///   --weight 50 \
    ///   --mode accept
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     nodebalancers:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs/{}/nodes", & request.api_version, &
                    request.node_balancer_id, & request.config_id
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponse,
        >(response)
            .await
    }
    /// Updates information about a Node, a backend for this NodeBalancer's configured port.
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
    ///     linode-cli nodebalancers node-update \
    ///   12345 4567 54321 \
    ///   --address 192.168.210.120:80 \
    ///   --label node54321 \
    ///   --weight 50 \
    ///   --mode accept
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     nodebalancers:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs/{}/nodes/{}", & request.api_version, &
                    request.node_balancer_id, & request.config_id, & request.node_id
                ),
            );
        let mut builder = reqwest::Client::default().put(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdResponse,
        >(response)
            .await
    }
}
