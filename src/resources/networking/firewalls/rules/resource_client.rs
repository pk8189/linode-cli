#[derive(Clone, Debug)]
pub struct RulesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RulesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns the inbound and outbound Rules for a Firewall.
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
    ///     linode-cli firewalls rules-list 123
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
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdRulesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}/rules", & request.api_version, & request
                    .firewall_id
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
            crate::models::GetApiVersionNetworkingFirewallsFirewallIdRulesResponse,
        >(response)
            .await
    }
    /// Updates the inbound and outbound Rules for a Firewall.
    ///
    /// - Assigned Linodes must not have any ongoing live migrations.
    ///
    /// - __Note__. This operation replaces all of a Firewall's `inbound` and `outbound` rulesets with the values specified in your request.
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
    ///     linode-cli firewalls rules-update 123 \
    ///   --inbound '[{"action":"ACCEPT", "protocol": "TCP", "ports": "22, 80, 8080, 443", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"], "ipv6": ["2001:DB8::/128"]}}]' \
    ///   --outbound '[{"action":"DROP","protocol": "TCP", "ports": "49152-65535", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"], "ipv6": ["2001:DB8::/128`"]}}]'
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     firewall:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionNetworkingFirewallsFirewallIdRulesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}/rules", & request.api_version, & request
                    .firewall_id
                ),
            );
        let mut builder = reqwest::Client::default().put(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PutApiVersionNetworkingFirewallsFirewallIdRulesResponse,
        >(response)
            .await
    }
}
