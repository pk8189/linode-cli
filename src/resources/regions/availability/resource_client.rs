#[derive(Clone, Debug)]
pub struct AvailabilityClient {
    base_client: crate::core::base_client::BaseClient,
}
impl AvailabilityClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns availability data for all Regions.
    ///
    /// Currently, this operation returns availability of select premium and GPU plans for select regions.
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
    ///     linode-cli regions list-avail
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(&format!("/{}/regions/availability", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
    /// Returns availability data for a single Region.
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
    ///     linode-cli regions view-avail us-east
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list_1(
        &self,
        request: super::request_types::List1Request,
    ) -> crate::SdkResult<
        Vec<crate::models::GetApiVersionRegionsRegionIdAvailabilityResponseItem>,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/regions/{}/availability", & request.api_version, & request
                    .region_id
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            Vec<crate::models::GetApiVersionRegionsRegionIdAvailabilityResponseItem>,
        >(response)
            .await
    }
}
