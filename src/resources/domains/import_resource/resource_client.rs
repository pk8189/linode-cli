#[derive(Clone, Debug)]
pub struct ImportClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ImportClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Imports a domain zone from a remote nameserver. Your nameserver must allow zone transfers (AXFR) from the following IPs:
    ///
    /// - 96.126.114.97
    /// - 96.126.114.98
    /// - 2600:3c00::5e
    /// - 2600:3c00::5f
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
    ///     linode-cli domains import --domain example.com --remote_nameserver examplenameserver.com
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
    ) -> crate::SdkResult<crate::models::PostApiVersionDomainsImportResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/domains/import", & request.api_version));
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
            crate::models::PostApiVersionDomainsImportResponse,
        >(response)
            .await
    }
}
