#[derive(Clone, Debug)]
pub struct TfaEnableClient {
    base_client: crate::core::base_client::BaseClient,
}
impl TfaEnableClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Generates a Two Factor secret for your User. To enable TFA for your User, enter the secret obtained from this operation with the [Enable two factor authentication](https://techdocs.akamai.com/linode-api/reference/post-tfa-confirm) operation. Once enabled, logins from untrusted computers are required to provide a TFA code before they are successful.
    ///
    /// Run the [Answer security questions](https://techdocs.akamai.com/linode-api/reference/post-security-questions) operation.
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
    ///     linode-cli profile tfa-enable
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
    ) -> crate::SdkResult<crate::models::PostApiVersionProfileTfaEnableResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/profile/tfa-enable", & request.api_version));
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostApiVersionProfileTfaEnableResponse,
        >(response)
            .await
    }
}
