#[derive(Clone, Debug)]
pub struct EventsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl EventsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn read(
        &self,
    ) -> crate::resources::account::events::read::resource_client::ReadClient {
        crate::resources::account::events::read::resource_client::ReadClient::new(
            self.base_client.clone(),
        )
    }
    pub fn seen(
        &self,
    ) -> crate::resources::account::events::seen::resource_client::SeenClient {
        crate::resources::account::events::seen::resource_client::SeenClient::new(
            self.base_client.clone(),
        )
    }
    /// Returns a collection of Event objects representing actions taken on your Account from the last 90 days. The Events returned depend on your grants.
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
    ///     linode-cli events list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     events:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionAccountEventsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/events", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("page", &request.page, false);
        queries.add_option("page_size", &request.page_size, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionAccountEventsResponse,
        >(response)
            .await
    }
    /// Returns a single Event object.
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
    ///     linode-cli events view 123
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     events:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionAccountEventsEventIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/events/{}", & request.api_version, & request.event_id
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionAccountEventsEventIdResponse,
        >(response)
            .await
    }
}
