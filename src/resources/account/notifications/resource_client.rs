#[derive(Clone, Debug)]
pub struct NotificationsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl NotificationsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns a collection of notification objects that represent important, often time-sensitive details about your account. You can't interact directly with notifications, and a notification disappears when the circumstances that caused it have been resolved. For example, if you have an important ticket open, you can respond to that ticket to dismiss its notification.
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
    ///     linode-cli account notifications-list
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
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionAccountNotificationsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/notifications", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionAccountNotificationsResponse,
        >(response)
            .await
    }
}
