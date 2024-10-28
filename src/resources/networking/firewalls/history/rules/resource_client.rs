#[derive(Clone, Debug)]
pub struct RulesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RulesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Get a specific firewall rule version for an `enabled` or `disabled` firewall.
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
    ///     linode-cli firewalls version-view \
    ///   123 2
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     firewall:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}/history/rules/{}", & request
                    .api_version, & request.firewall_id, & request.version
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
            crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionResponse,
        >(response)
            .await
    }
}
