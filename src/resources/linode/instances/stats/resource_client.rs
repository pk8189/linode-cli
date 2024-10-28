#[derive(Clone, Debug)]
pub struct StatsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl StatsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns CPU, IO, IPv4, and IPv6 statistics for your Linode for the past 24 hours.
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
    ///     linodes:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/stats", & request.api_version, & request
                    .linode_id
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
            crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponse,
        >(response)
            .await
    }
    /// Returns statistics for a specific month. The year/month values must be either a date in the past, or the current month. If the current month, statistics will be retrieved for the past 30 days.
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
    ///     linodes:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLinodeInstancesLinodeIdStatsYearMonthResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/stats/{}/{}", & request.api_version, &
                    request.linode_id, & request.year, & request.month
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
            crate::models::GetApiVersionLinodeInstancesLinodeIdStatsYearMonthResponse,
        >(response)
            .await
    }
}
