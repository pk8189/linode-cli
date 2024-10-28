#[derive(Clone, Debug)]
pub struct FirewallsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl FirewallsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn devices(
        &self,
    ) -> crate::resources::networking::firewalls::devices::resource_client::DevicesClient {
        crate::resources::networking::firewalls::devices::resource_client::DevicesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn history(
        &self,
    ) -> crate::resources::networking::firewalls::history::resource_client::HistoryClient {
        crate::resources::networking::firewalls::history::resource_client::HistoryClient::new(
            self.base_client.clone(),
        )
    }
    pub fn rules(
        &self,
    ) -> crate::resources::networking::firewalls::rules::resource_client::RulesClient {
        crate::resources::networking::firewalls::rules::resource_client::RulesClient::new(
            self.base_client.clone(),
        )
    }
    /// Delete a Firewall resource by its ID. This removes all of the Firewall's Rules from any services that the Firewall was assigned to.
    ///
    /// - Assigned Linodes must not have any ongoing live migrations.
    ///
    /// - A `firewall_delete` Event is generated when this operation returns successfully.
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
    ///     linode-cli firewalls delete 123
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
    pub async fn delete(
        &self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}", & request.api_version, & request
                    .firewall_id
                ),
            );
        let mut builder = reqwest::Client::default().delete(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
    /// Returns a paginated list of accessible Firewalls.
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
    ///     linode-cli firewalls list
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
    ) -> crate::SdkResult<crate::models::GetApiVersionNetworkingFirewallsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/networking/firewalls", & request.api_version));
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
            crate::models::GetApiVersionNetworkingFirewallsResponse,
        >(response)
            .await
    }
    /// Get a specific Firewall resource by its ID. The Firewall's Devices will not be returned in the response. Instead, run the [List firewall devices](https://techdocs.akamai.com/linode-api/reference/get-firewall-devices) operation to review them.
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
    ///     linode-cli firewalls view 123
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
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}", & request.api_version, & request
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
            crate::models::GetApiVersionNetworkingFirewallsFirewallIdResponse,
        >(response)
            .await
    }
    /// Creates a Firewall to filter network traffic.
    ///
    /// - Use `rules` to create inbound and outbound access rules. Rule versions increment from `1` whenever the firewall's `rules` change.
    ///
    /// - Use `devices` to assign the firewall to a service and apply its rules to the device. Requires `read_write` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) to the device. Currently, firewalls can be assigned to Linode compute instances and NodeBalancers.
    ///
    /// - A Firewall can be assigned to multiple services at a time.
    ///
    /// - Use `firewall_id` to assign a firewall when [creating a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance).
    ///
    /// - A service can have one assigned Firewall at a time.
    ///
    /// - Firewalls apply to all of a Linode's non-`vlan` purpose Configuration Profile Interfaces.
    ///
    /// - Assigned Linodes must not have any ongoing live migrations.
    ///
    /// - A `firewall_create` Event is generated when this operation succeeds.
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
    ///     linode-cli firewalls create \
    ///   --label example-firewall \
    ///   --rules.outbound_policy ACCEPT \
    ///   --rules.inbound_policy DROP \
    ///   --rules.inbound '[{"protocol": "TCP", "ports": "22, 80, 8080, 443", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"], "ipv6": ["2001:DB8::/128"]}, "action": "ACCEPT"}]' \
    ///   --rules.outbound '[{"protocol": "TCP", "ports": "49152-65535", "addresses": {"ipv4": ["192.0.2.0/24", "198.51.100.2/32"],"ipv6": ["2001:DB8::/128"]}, "action": "DROP", "label": "outbound-rule123", "description": "An example outbound rule description."}]'
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
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostApiVersionNetworkingFirewallsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/networking/firewalls", & request.api_version));
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostApiVersionNetworkingFirewallsResponse,
        >(response)
            .await
    }
    /// Updates information for a Firewall.
    ///
    /// - Assigned Linodes must not have any ongoing live migrations.
    ///
    /// - If a Firewall's status is changed with this operation, a corresponding `firewall_enable` or `firewall_disable` Event will be generated.
    ///
    /// Some parts of a Firewall's configuration cannot be manipulated by this operation:
    ///
    /// - A Firewall's Devices cannot be set with this operation. Instead, run the [Create a firewall device](https://techdocs.akamai.com/linode-api/reference/post-firewall-device) and [Delete a firewall device](https://techdocs.akamai.com/linode-api/reference/delete-firewall-device) operations to assign and remove this Firewall from services.
    ///
    /// - A Firewall's Rules cannot be changed with this operation. Instead, run the [Update firewall rules](https://techdocs.akamai.com/linode-api/reference/put-firewall-rules) operation to update your Rules.
    ///
    /// - A Firewall's status can be set to `enabled` or `disabled` by this operation, but it cannot be set to `deleted`. Instead, run the [Delete a firewall](https://techdocs.akamai.com/linode-api/reference/delete-firewall) operation to delete a Firewall.
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
    ///     linode-cli firewalls update 123 \
    ///   --status disabled
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
        crate::models::PutApiVersionNetworkingFirewallsFirewallIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}", & request.api_version, & request
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
            crate::models::PutApiVersionNetworkingFirewallsFirewallIdResponse,
        >(response)
            .await
    }
}
