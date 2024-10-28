#[derive(Clone, Debug)]
pub struct PoolsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl PoolsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn recycle(
        &self,
    ) -> crate::resources::lke::clusters::pools::recycle::resource_client::RecycleClient {
        crate::resources::lke::clusters::pools::recycle::resource_client::RecycleClient::new(
            self.base_client.clone(),
        )
    }
    /// Delete a specific Node Pool from a Kubernetes cluster.
    ///
    /// __Deleting a Node Pool is a destructive action and cannot be undone.__
    ///
    /// Deleting a Node Pool will delete all Linodes within that Pool.
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
    ///     linode-cli lke pool-delete 12345 456
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
                    "/{}/lke/clusters/{}/pools/{}", & request.api_version, & request
                    .cluster_id, & request.pool_id
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
    /// Returns all active Node Pools on a Kubernetes cluster.
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
    ///     linode-cli lke pools-list 12345
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
        crate::models::GetApiVersionLkeClustersClusterIdPoolsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}/pools", & request.api_version, & request
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
            crate::models::GetApiVersionLkeClustersClusterIdPoolsResponse,
        >(response)
            .await
    }
    /// Get a specific Node Pool by ID.
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
    ///     linode-cli lke pool-view 12345 456
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
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}/pools/{}", & request.api_version, & request
                    .cluster_id, & request.pool_id
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
            crate::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdResponse,
        >(response)
            .await
    }
    /// Creates a new Node Pool for the designated Kubernetes cluster.
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
    ///     linode-cli lke pool-create 12345 \
    ///   --type g6-standard-4 \
    ///   --count 6 \
    ///   --tags example-tag \
    ///   --autoscaler.enabled true \
    ///   --autoscaler.max 12 \
    ///   --autoscaler.min 3 \
    ///   --labels.key "example.com/my-app" \
    ///   --labels.value "teams" \
    ///   --taints.effect "NoSchedule" \
    ///   --taints.key "example.com/my-app" \
    ///   --taints.value "teamA"
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
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionLkeClustersClusterIdPoolsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}/pools", & request.api_version, & request
                    .cluster_id
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
            crate::models::PostApiVersionLkeClustersClusterIdPoolsResponse,
        >(response)
            .await
    }
    /// Updates a node pool's count, labels and taints, and autoscaler configuration.
    ///
    /// Linodes are created or deleted to match changes to the Node Pool's count.
    ///
    /// Specifying labels or taints on update overwrites any previous values, and updates existing nodes with the new values without a recycle.
    ///
    /// __Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__
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
    ///     linode-cli lke pool-update 12345 456 \
    ///   --count 6 \
    ///   --autoscaler.enabled true \
    ///   --autoscaler.max 12 \
    ///   --autoscaler.min 3 \
    ///   --labels.key "example.com/my-app" \
    ///   --labels.value "teams" \
    ///   --taints.effect "NoSchedule" \
    ///   --taints.key "example.com/my-app" \
    ///   --taints.value "teamA"
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
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}/pools/{}", & request.api_version, & request
                    .cluster_id, & request.pool_id
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
            crate::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdResponse,
        >(response)
            .await
    }
}
