#[derive(Clone, Debug)]
pub struct CloneClient {
    base_client: crate::core::base_client::BaseClient,
}
impl CloneClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Clones a Domain and all associated DNS records from a Domain that is registered in Linode's DNS manager.
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
    ///     linode-cli domains clone 123 --domain example.com
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     domains:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostApiVersionDomainsDomainIdCloneResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/domains/{}/clone", & request.api_version, & request.domain_id
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
            crate::models::PostApiVersionDomainsDomainIdCloneResponse,
        >(response)
            .await
    }
}
