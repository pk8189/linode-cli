#[derive(Clone, Debug)]
pub struct DevicesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl DevicesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Removes a Firewall Device, which removes a Firewall from the service it was assigned to by the Device. This removes all of the Firewall's Rules from the service. If any other Firewalls have been assigned to the service, then those Rules remain in effect.
    ///
    /// - Assigned Linodes must not have any ongoing live migrations.
    ///
    /// - A `firewall_device_remove` Event is generated when the Firewall Device is removed successfully.
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
    ///     linode-cli firewalls device-delete 123 456
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
                    "/{}/networking/firewalls/{}/devices/{}", & request.api_version, &
                    request.firewall_id, & request.device_id
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
    /// Returns a paginated list of a Firewall's Devices. A Firewall Device assigns a Firewall to a service (referred to as the Device's `entity`).
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
    ///     linode-cli firewalls devices-list 123
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
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}/devices", & request.api_version, &
                    request.firewall_id
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
            crate::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesResponse,
        >(response)
            .await
    }
    /// Returns information for a Firewall Device, which assigns a Firewall to a service (referred to as the Device's `entity`).
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
    ///     linode-cli firewalls device-view \
    ///   123 456
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
        crate::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}/devices/{}", & request.api_version, &
                    request.firewall_id, & request.device_id
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
            crate::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdResponse,
        >(response)
            .await
    }
    /// Creates a Firewall Device, which assigns a Firewall to a service (referred to as the Device's `entity`) and applies the Firewall's Rules to the device.
    ///
    /// - Currently, Devices with `linode` and `nodebalancer` entity types are accepted.
    ///
    /// - Firewalls only apply to inbound TCP traffic to NodeBalancers.
    ///
    /// - A Firewall can be assigned to multiple services at a time.
    ///
    /// - A service can have one assigned Firewall at a time.
    ///
    /// - Assigned Linodes must not have any ongoing live migrations.
    ///
    /// - A `firewall_device_add` Event is generated when the Firewall Device is added successfully.
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
    ///     linode-cli firewalls device-create 123 \
    ///   --id 456 \
    ///   --type "linode"
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
    ) -> crate::SdkResult<
        crate::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/networking/firewalls/{}/devices", & request.api_version, &
                    request.firewall_id
                ),
            );
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
            crate::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesResponse,
        >(response)
            .await
    }
}
