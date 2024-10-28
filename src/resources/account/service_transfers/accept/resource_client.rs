#[derive(Clone, Debug)]
pub struct AcceptClient {
    base_client: crate::core::base_client::BaseClient,
}
impl AcceptClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Accept a Service Transfer for the provided token to receive the services included in the transfer to your account. At this time, only Linodes can be transferred.
    ///
    /// When accepted, email confirmations are sent to the accounts that created and accepted the transfer. A transfer can take up to three hours to complete once accepted. Once a transfer is completed, billing for transferred services ends for the sending account and begins for the receiving account.
    ///
    /// This operation can only be accessed by the unrestricted users of the account that receives the transfer. Users of the same account that created a transfer cannot accept the transfer.
    ///
    /// There are several conditions that must be met in order to accept a transfer request:
    ///
    /// 1. Only transfers with a `pending` status can be accepted.
    ///
    /// 1. The account accepting the transfer must have a registered payment method and must not have a past due balance or other account limitations for the services to be transferred.
    ///
    /// 1. Both the account that created the transfer and the account that is accepting the transfer must not have any active Terms of Service violations.
    ///
    /// 1. The service must still be owned by the account that created the transfer.
    ///
    /// 1. Linodes must not:
    ///
    ///     - be assigned to a NodeBalancer, Firewall, VLAN, or Managed Service.
    ///
    ///     - have any attached Block Storage Volumes.
    ///
    ///     - have any shared IP addresses.
    ///
    ///     - have any assigned /56, /64, or /116 IPv6 ranges.
    ///
    /// Any and all of the above conditions must be cured and maintained by the relevant account prior to the transfer's expiration to allow the transfer to be accepted by the receiving account.
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
    ///     linode-cli service-transfers \
    ///   accept 123E4567-E89B-12D3-A456-426614174000
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/service-transfers/{}/accept", & request.api_version, &
                    request.token
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
}
