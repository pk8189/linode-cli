#[derive(Clone, Debug)]
pub struct AssignClient {
    base_client: crate::core::base_client::BaseClient,
}
impl AssignClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Assign multiple IPv4 addresses and/or IPv6 ranges to multiple Linodes in one Region. This allows swapping, shuffling, or otherwise reorganizing IPs to your Linodes.
    ///
    /// The following restrictions apply:
    ///
    /// - All Linodes involved must have at least one public IPv4 address after assignment.
    /// - Linodes may have no more than one assigned private IPv4 address.
    /// - Linodes may have no more than one assigned IPv6 range.
    /// - Shared IP addresses cannot be swapped between Linodes.
    ///
    /// [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to request additional IPv4 addresses or IPv6 ranges beyond standard account limits.
    ///
    /// __Note__. Removing an IP address that has been set as a Managed Linode's `ssh.ip` causes the Managed Linode's SSH access settings to reset to their default values.
    ///
    /// To view and configure Managed Linode SSH settings, use the following operations:
    ///
    /// - [Get a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/get-managed-linode-setting)
    /// - [Update a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/put-managed-linode-setting)
    ///
    /// __Note__. Addresses with an active 1:1 NAT to a VPC Interface address cannot be assigned to other Linodes.
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
    ///     linode-cli networking ip-assign \
    ///   --region us-east \
    ///   --assignments.address 192.0.2.1 \
    ///   --assignments.linode_id 123 \
    ///   --assignments.address 2001:db8:3c4d:15::/64 \
    ///   --assignments.linode_id 234
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     ips:read_write
    /// linodes:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(&format!("/{}/networking/ips/assign", & request.api_version));
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
}
