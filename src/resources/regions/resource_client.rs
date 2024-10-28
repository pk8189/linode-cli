#[derive(Clone, Debug)]
pub struct RegionsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RegionsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn availability(
        &self,
    ) -> crate::resources::regions::availability::resource_client::AvailabilityClient {
        crate::resources::regions::availability::resource_client::AvailabilityClient::new(
            self.base_client.clone(),
        )
    }
    /// Lists the Regions available for Linode services. Not all services are guaranteed to be available in all Regions.
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
    ///     linode-cli regions list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionRegionsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/regions", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionRegionsResponse,
        >(response)
            .await
    }
    /// Returns a single Region.
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
    ///     linode-cli regions view us-east
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionRegionsRegionIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/regions/{}", & request.api_version, & request.region_id),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionRegionsRegionIdResponse,
        >(response)
            .await
    }
}
