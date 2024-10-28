#[derive(Clone, Debug)]
pub struct FirewallsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl FirewallsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn apply(
        &self,
    ) -> crate::resources::linode::instances::firewalls::apply::resource_client::ApplyClient {
        crate::resources::linode::instances::firewalls::apply::resource_client::ApplyClient::new(
            self.base_client.clone(),
        )
    }
    /// View Firewall information for Firewalls assigned to this Linode.
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
    ///     linode-cli linodes firewalls-list 123
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
        crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/firewalls", & request.api_version, & request
                    .linode_id
                ),
            );
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
            crate::models::GetApiVersionLinodeInstancesLinodeIdFirewallsResponse,
        >(response)
            .await
    }
}
