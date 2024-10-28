#[derive(Clone, Debug)]
pub struct PricesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl PricesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns collection of network transfer prices, including any region-specific rates.
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
    ///     linode-cli network-transfer prices
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionNetworkTransferPricesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/network-transfer/prices", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionNetworkTransferPricesResponse,
        >(response)
            .await
    }
}
