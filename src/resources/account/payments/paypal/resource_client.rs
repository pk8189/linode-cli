#[derive(Clone, Debug)]
pub struct PaypalClient {
    base_client: crate::core::base_client::BaseClient,
}
impl PaypalClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn execute(
        &self,
    ) -> crate::resources::account::payments::paypal::execute::resource_client::ExecuteClient {
        crate::resources::account::payments::paypal::execute::resource_client::ExecuteClient::new(
            self.base_client.clone(),
        )
    }
    /// __Deprecated__ This operation is disabled and no longer accessible. PayPal can be designated as a Payment Method for automated payments using the Cloud Manager. See [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/).
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
    ) -> crate::SdkResult<
        crate::models::PostApiVersionAccountPaymentsPaypalResponse1000,
    > {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/payments/paypal", & request.api_version));
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
            crate::models::PostApiVersionAccountPaymentsPaypalResponse1000,
        >(response)
            .await
    }
}
