#[derive(Clone, Debug)]
pub struct NodebalancersClient {
    base_client: crate::core::base_client::BaseClient,
}
impl NodebalancersClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn configs(
        &self,
    ) -> crate::resources::nodebalancers::configs::resource_client::ConfigsClient {
        crate::resources::nodebalancers::configs::resource_client::ConfigsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn types_resource(
        &self,
    ) -> crate::resources::nodebalancers::types_resource::resource_client::TypesClient {
        crate::resources::nodebalancers::types_resource::resource_client::TypesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn firewalls(
        &self,
    ) -> crate::resources::nodebalancers::firewalls::resource_client::FirewallsClient {
        crate::resources::nodebalancers::firewalls::resource_client::FirewallsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn stats(
        &self,
    ) -> crate::resources::nodebalancers::stats::resource_client::StatsClient {
        crate::resources::nodebalancers::stats::resource_client::StatsClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a NodeBalancer.
    ///
    /// __This is a destructive action and cannot be undone.__
    ///
    /// Deleting a NodeBalancer will also delete all associated Configs and Nodes, although the backend servers represented by the Nodes will not be changed or removed. Deleting a NodeBalancer will cause you to lose access to the IP Addresses assigned to this NodeBalancer.
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
    ///     linode-cli nodebalancers delete 12345
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
                    "/{}/nodebalancers/{}", & request.api_version, & request
                    .node_balancer_id
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
    /// Returns a paginated list of NodeBalancers you have access to.
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
    ///     linode-cli nodebalancers list
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
    ) -> crate::SdkResult<crate::models::GetApiVersionNodebalancersResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/nodebalancers", & request.api_version));
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
            crate::models::GetApiVersionNodebalancersResponse,
        >(response)
            .await
    }
    /// Returns a single NodeBalancer you can access.
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
    ///     linode-cli nodebalancers view 12345
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
        crate::models::GetApiVersionNodebalancersNodeBalancerIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}", & request.api_version, & request
                    .node_balancer_id
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
            crate::models::GetApiVersionNodebalancersNodeBalancerIdResponse,
        >(response)
            .await
    }
    /// Creates a NodeBalancer in the requested Region. Only available in [regions](https://techdocs.akamai.com/linode-api/reference/get-regions) with "NodeBalancers" in their `capabilities`.
    ///
    /// NodeBalancers require a port Config with at least one backend Node to start serving requests.
    ///
    /// When using the Linode CLI to create a NodeBalancer, first create a NodeBalancer without any Configs. Then, create Configs and Nodes for that NodeBalancer with the respective [Create a config](https://techdocs.akamai.com/linode-api/reference/post-node-balancer-config) and [Create a node](https://techdocs.akamai.com/linode-api/reference/post-node-balancer-node) operations.
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
    ///     linode-cli nodebalancers create \
    ///   --region us-east \
    ///   --label balancer12345 \
    ///   --client_conn_throttle 0
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
    ) -> crate::SdkResult<crate::models::PostApiVersionNodebalancersResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/nodebalancers", & request.api_version));
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
            crate::models::PostApiVersionNodebalancersResponse,
        >(response)
            .await
    }
    /// Updates information about a NodeBalancer you can access.
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
    ///     linode-cli nodebalancers update 12345 \
    ///   --label balancer12345 \
    ///   --client_conn_throttle 0
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
        crate::models::PutApiVersionNodebalancersNodeBalancerIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}", & request.api_version, & request
                    .node_balancer_id
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
            crate::models::PutApiVersionNodebalancersNodeBalancerIdResponse,
        >(response)
            .await
    }
}
