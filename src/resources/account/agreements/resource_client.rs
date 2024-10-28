#[derive(Clone, Debug)]
pub struct AgreementsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl AgreementsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns all agreements and their acceptance status for your account.
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
    ///     account:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionAccountAgreementsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/agreements", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionAccountAgreementsResponse,
        >(response)
            .await
    }
    /// Accept required agreements by setting them to `true`. This remains until the content of the agreement changes. If it does, you need to run this operation again to accept it. If you set this to `false`, the API rejects the request and you need to open a [support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to reset the agreement. Omitted agreements are left unchanged.
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
    ///     account:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/agreements", & request.api_version));
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
}
