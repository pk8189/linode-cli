#[derive(Clone, Debug)]
pub struct ClustersClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ClustersClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn control_plane_acl(
        &self,
    ) -> crate::resources::lke::clusters::control_plane_acl::resource_client::ControlPlaneAclClient {
        crate::resources::lke::clusters::control_plane_acl::resource_client::ControlPlaneAclClient::new(
            self.base_client.clone(),
        )
    }
    pub fn kubeconfig(
        &self,
    ) -> crate::resources::lke::clusters::kubeconfig::resource_client::KubeconfigClient {
        crate::resources::lke::clusters::kubeconfig::resource_client::KubeconfigClient::new(
            self.base_client.clone(),
        )
    }
    pub fn nodes(
        &self,
    ) -> crate::resources::lke::clusters::nodes::resource_client::NodesClient {
        crate::resources::lke::clusters::nodes::resource_client::NodesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn pools(
        &self,
    ) -> crate::resources::lke::clusters::pools::resource_client::PoolsClient {
        crate::resources::lke::clusters::pools::resource_client::PoolsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn servicetoken(
        &self,
    ) -> crate::resources::lke::clusters::servicetoken::resource_client::ServicetokenClient {
        crate::resources::lke::clusters::servicetoken::resource_client::ServicetokenClient::new(
            self.base_client.clone(),
        )
    }
    pub fn api_endpoints(
        &self,
    ) -> crate::resources::lke::clusters::api_endpoints::resource_client::ApiEndpointsClient {
        crate::resources::lke::clusters::api_endpoints::resource_client::ApiEndpointsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn dashboard(
        &self,
    ) -> crate::resources::lke::clusters::dashboard::resource_client::DashboardClient {
        crate::resources::lke::clusters::dashboard::resource_client::DashboardClient::new(
            self.base_client.clone(),
        )
    }
    pub fn recycle(
        &self,
    ) -> crate::resources::lke::clusters::recycle::resource_client::RecycleClient {
        crate::resources::lke::clusters::recycle::resource_client::RecycleClient::new(
            self.base_client.clone(),
        )
    }
    pub fn regenerate(
        &self,
    ) -> crate::resources::lke::clusters::regenerate::resource_client::RegenerateClient {
        crate::resources::lke::clusters::regenerate::resource_client::RegenerateClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a Cluster you have permission to `read_write`.
    ///
    /// __Deleting a Cluster is a destructive action and cannot be undone.__
    ///
    /// Deleting a Cluster:
    ///
    /// - Deletes all Linodes in all pools within this Kubernetes cluster
    /// - Deletes all supporting Kubernetes services for this Kubernetes cluster (API server, etcd, etc)
    /// - Deletes all NodeBalancers created by this Kubernetes cluster
    /// - Does not delete any of the volumes created by this Kubernetes cluster
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
    ///     linode-cli lke cluster-delete 12345
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
                    "/{}/lke/clusters/{}", & request.api_version, & request.cluster_id
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
    /// Lists current Kubernetes clusters available on your account.
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
    ///     linode-cli lke clusters-list
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
    ) -> crate::SdkResult<crate::models::GetApiVersionLkeClustersResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/lke/clusters", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionLkeClustersResponse,
        >(response)
            .await
    }
    /// Get a specific Cluster by ID.
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
    ///     linode-cli lke cluster-view 12345
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
    ) -> crate::SdkResult<crate::models::GetApiVersionLkeClustersClusterIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}", & request.api_version, & request.cluster_id
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
            crate::models::GetApiVersionLkeClustersClusterIdResponse,
        >(response)
            .await
    }
    /// Creates a Kubernetes cluster. The Kubernetes cluster will be created asynchronously. You can use the events system to determine when the Kubernetes cluster is ready to use. Please note that it often takes 2-5 minutes before the [Kubernetes API endpoints](https://techdocs.akamai.com/linode-api/reference/get-lke-cluster-api-endpoints) and the [Kubeconfig file](https://techdocs.akamai.com/linode-api/reference/get-lke-cluster-kubeconfig) for the new cluster are ready.
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
    ///     linode-cli lke cluster-create \
    ///   --label cluster12345 \
    ///   --region us-central \
    ///   --k8s_version 1.27 \
    ///   --control_plane.high_availability true \
    ///   --node_pools.type g6-standard-4 --node_pools.count 6 \
    ///   --node_pools.type g6-standard-8 --node_pools.count 3 \
    ///   --node_pools.autoscaler.enabled true \
    ///   --node_pools.autoscaler.max 12 \
    ///   --node_pools.autoscaler.min 3 \
    ///   --tags ecomm
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
    ) -> crate::SdkResult<crate::models::PostApiVersionLkeClustersResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/lke/clusters", & request.api_version));
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
            crate::models::PostApiVersionLkeClustersResponse,
        >(response)
            .await
    }
    /// Updates a Kubernetes cluster.
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
    ///     linode-cli lke cluster-update 12345 \
    ///   --label lkecluster54321 \
    ///   --control_plane.high_availability true \
    ///   --k8s_version 1.27 \
    ///   --tags ecomm \
    ///   --tags blog \
    ///   --tags prod \
    ///   --tags monitoring
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
    ) -> crate::SdkResult<crate::models::PutApiVersionLkeClustersClusterIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/lke/clusters/{}", & request.api_version, & request.cluster_id
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
            crate::models::PutApiVersionLkeClustersClusterIdResponse,
        >(response)
            .await
    }
}
