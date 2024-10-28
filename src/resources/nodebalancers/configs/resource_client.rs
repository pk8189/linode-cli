#[derive(Clone, Debug)]
pub struct ConfigsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ConfigsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn nodes(
        &self,
    ) -> crate::resources::nodebalancers::configs::nodes::resource_client::NodesClient {
        crate::resources::nodebalancers::configs::nodes::resource_client::NodesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn rebuild(
        &self,
    ) -> crate::resources::nodebalancers::configs::rebuild::resource_client::RebuildClient {
        crate::resources::nodebalancers::configs::rebuild::resource_client::RebuildClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes the Config for a port of this NodeBalancer.
    ///
    /// __This cannot be undone.__
    ///
    /// Once completed, this NodeBalancer will no longer respond to requests on the given port. This also deletes all associated NodeBalancerNodes, but the Linodes they were routing traffic to will be unchanged and will not be removed.
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
    ///     linode-cli nodebalancers config-delete \
    ///   12345 4567
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
                    "/{}/nodebalancers/{}/configs/{}", & request.api_version, & request
                    .node_balancer_id, & request.config_id
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
    /// Returns a paginated list of NodeBalancer Configs associated with this NodeBalancer. NodeBalancer Configs represent individual ports that this NodeBalancer will accept traffic on, one Config per port.
    ///
    /// For example, if you wanted to accept standard HTTP traffic, you would need a Config listening on port 80.
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
    ///     linode-cli nodebalancers configs-list 12345
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
        crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs", & request.api_version, & request
                    .node_balancer_id
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
            crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsResponse,
        >(response)
            .await
    }
    /// Returns configuration information for a single port of this NodeBalancer.
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
    ///     linode-cli nodebalancers config-view \
    ///   12345 4567
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
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs/{}", & request.api_version, & request
                    .node_balancer_id, & request.config_id
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
            crate::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponse,
        >(response)
            .await
    }
    /// Creates a NodeBalancer Config, which allows the NodeBalancer to accept traffic on a new port. You will need to add NodeBalancer Nodes to the new Config before it can actually serve requests.
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
    ///     linode-cli nodebalancers config-create 12345 \
    ///   --port 443 \
    ///   --protocol https \
    ///   --algorithm roundrobin \
    ///   --stickiness http_cookie \
    ///   --check http_body \
    ///   --check_interval 90 \
    ///   --check_timeout 10 \
    ///   --check_attempts 3 \
    ///   --check_path "/test" \
    ///   --check_body "it works" \
    ///   --check_passive true \
    ///   --proxy_protocol "none" \
    ///   --ssl_cert "-----BEGIN CERTIFICATE-----
    ///               CERTIFICATE_INFORMATION
    ///               -----END CERTIFICATE-----" \
    ///   --ssl_key "-----BEGIN PRIVATE KEY-----
    ///              PRIVATE_KEY_INFORMATION
    ///              -----END PRIVATE KEY-----" \
    ///   --cipher_suite recommended
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
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs", & request.api_version, & request
                    .node_balancer_id
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
            crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsResponse,
        >(response)
            .await
    }
    /// Updates the configuration for a single port on a NodeBalancer.
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
    ///     linode-cli nodebalancers config-update \
    ///   12345 4567 \
    ///   --port 443 \
    ///   --protocol https \
    ///   --algorithm roundrobin \
    ///   --stickiness http_cookie \
    ///   --check http_body \
    ///   --check_interval 90 \
    ///   --check_timeout 10 \
    ///   --check_attempts 3 \
    ///   --check_path "/test" \
    ///   --check_body "it works" \
    ///   --check_passive true \
    ///   --proxy_protocol "none" \
    ///   --ssl_cert "-----BEGIN CERTIFICATE-----
    ///               CERTIFICATE_INFORMATION
    ///               -----END CERTIFICATE-----" \
    ///   --ssl_key "-----BEGIN PRIVATE KEY-----
    ///              PRIVATE_KEY_INFORMATION
    ///              -----END PRIVATE KEY-----" \
    ///   --cipher_suite recommended
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
        crate::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs/{}", & request.api_version, & request
                    .node_balancer_id, & request.config_id
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
            crate::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponse,
        >(response)
            .await
    }
}
