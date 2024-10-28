#[derive(Clone, Debug)]
pub struct PoolsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl PoolsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Displays the IPv6 pools on your Account. A pool of IPv6 addresses are routed to all of your Linodes in a single [region](https://techdocs.akamai.com/linode-api/reference/get-regions). Any Linode on your Account may bring up any address in this pool at any time, with no external configuration required.
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
    ///     linode-cli networking v6-pools
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     ips:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionNetworkingIpv6PoolsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/networking/ipv6/pools", & request.api_version));
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
            crate::models::GetApiVersionNetworkingIpv6PoolsResponse,
        >(response)
            .await
    }
}
