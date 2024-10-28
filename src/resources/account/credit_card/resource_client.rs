#[derive(Clone, Debug)]
pub struct CreditCardClient {
    base_client: crate::core::base_client::BaseClient,
}
impl CreditCardClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// __Deprecated__ Please run [Add a payment method](https://techdocs.akamai.com/linode-api/reference/post-payment-method).
    ///
    /// Adds a credit card Payment Method to your account and sets it as the default method.
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
            .build_url(&format!("/{}/account/credit-card", & request.api_version));
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
