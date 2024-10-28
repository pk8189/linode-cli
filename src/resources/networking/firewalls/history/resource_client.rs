#[derive(Clone, Debug)]
pub struct HistoryClient {
    base_client: crate::core::base_client::BaseClient,
}
impl HistoryClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn rules(
        &self,
    ) -> crate::resources::networking::firewalls::history::rules::resource_client::RulesClient {
        crate::resources::networking::firewalls::history::rules::resource_client::RulesClient::new(
            self.base_client.clone(),
        )
    }
    /// Lists the current and historical rules of the firewall (that is not deleted), using `version`. Whenever rules update, the `version` increments from `1`.
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
    ///     linode-cli firewalls versions-list 123
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
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}/history", & request.api_version, &
                    request.firewall_id
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
            crate::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryResponse,
        >(response)
            .await
    }
}
