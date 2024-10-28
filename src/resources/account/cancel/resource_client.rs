#[derive(Clone, Debug)]
pub struct CancelClient {
    base_client: crate::core::base_client::BaseClient,
}
impl CancelClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Cancels an active Linode account. Akamai attempts to charge the credit card on file for any remaining balance. An error occurs if this charge fails.
    ///
    /// __Note__. This operation can only be accessed by account users with _unrestricted_ access.
    ///
    /// __Parent and child accounts__ In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:
    ///
    /// - A child account user can't cancel a child account.
    /// - You can't cancel a parent account if it has an active child account.
    /// - You need to work with your Akamai account team to dissolve any parent-child account relationships before you can fully cancel a child or parent account.
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
    ///     linode-cli account cancel \
    ///   --comments "I'm consolidating my accounts"
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
    ) -> crate::SdkResult<crate::models::PostApiVersionAccountCancelResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/cancel", & request.api_version));
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
            crate::models::PostApiVersionAccountCancelResponse,
        >(response)
            .await
    }
}
