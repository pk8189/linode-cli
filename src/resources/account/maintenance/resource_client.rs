#[derive(Clone, Debug)]
pub struct MaintenanceClient {
    base_client: crate::core::base_client::BaseClient,
}
impl MaintenanceClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns a collection of Maintenance objects for any entity a user has permissions to view. Canceled Maintenance objects are not returned.
    ///
    /// Currently, Linodes are the only entities available for viewing.
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
    ///     linode-cli account maintenance-list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionAccountMaintenanceResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/maintenance", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionAccountMaintenanceResponse,
        >(response)
            .await
    }
}
