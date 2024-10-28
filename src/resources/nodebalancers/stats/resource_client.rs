#[derive(Clone, Debug)]
pub struct StatsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl StatsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns detailed statistics about the requested NodeBalancer.
    ///
    ///
    /// <<LB>>
    ///
    /// ---
    ///
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
        crate::models::GetApiVersionNodebalancersNodeBalancerIdStatsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/stats", & request.api_version, & request
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
            crate::models::GetApiVersionNodebalancersNodeBalancerIdStatsResponse,
        >(response)
            .await
    }
}
