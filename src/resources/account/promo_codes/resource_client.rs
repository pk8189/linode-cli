#[derive(Clone, Debug)]
pub struct PromoCodesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl PromoCodesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Adds an expiring Promo Credit to your account. The following restrictions apply:
    ///
    /// - Your account needs to be less than 90 days old.
    ///
    /// - You can't already have a Promo Credit on your account.
    ///
    /// - The user making the request needs to be unrestricted. You can run the [Update a user](https://techdocs.akamai.com/linode-api/reference/put-user) operation to change a user's restricted status.
    ///
    /// - The `promo_code` needs to be valid and unexpired.
    ///
    /// __Parent and child accounts__
    ///
    /// In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:
    ///
    /// - Child account users can't run this operation. These users don't have access to billing-related operations.
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
    ///     linode-cli account \
    ///   promo-add \
    ///   --promo-code abcdefABCDEF1234567890
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostApiVersionAccountPromoCodesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/promo-codes", & request.api_version));
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
            crate::models::PostApiVersionAccountPromoCodesResponse,
        >(response)
            .await
    }
}
