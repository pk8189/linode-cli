#[derive(Clone, Debug)]
pub struct CloneClient {
    base_client: crate::core::base_client::BaseClient,
}
impl CloneClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// You can clone your Linode's existing Disks or Configuration profiles to another Linode on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Cloning to a new Linode will incur a charge on your Account.
    ///
    /// If cloning to an existing Linode, any actions currently running or queued must be completed first before you can clone to it.
    ///
    /// Up to five clone operations from any given source Linode can be run concurrently. If more concurrent clones are attempted, an HTTP 400 error will be returned by this operation.
    ///
    /// Any [tags](https://techdocs.akamai.com/linode-api/reference/get-tags) existing on the source Linode will be cloned to the target Linode.
    ///
    /// Linodes utilizing Metadata (`"has_user_data": true`) must be cloned to a new Linode with `metadata.user_data` included with the clone request.
    ///
    /// `vpc` details
    ///
    /// - If the Linode you are cloning has a `vpc` purpose Interface on its active Configuration Profile that includes a 1:1 NAT, the resulting clone is configured with an `any` 1:1 NAT.
    /// - See the [VPC documentation](https://www.linode.com/docs/products/networking/vpc/#technical-specifications) guide for its specifications and limitations.
    ///
    /// `vlan` details
    ///
    /// - Only Next Generation Network (NGN) data centers support VLANs. If a VLAN is attached to your Linode and you attempt clone it to a non-NGN data center, the cloning will not initiate. If a Linode cannot be cloned because of an incompatibility, you will be prompted to select a different data center or contact support.
    /// - See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) guide to view additional specifications and limitations.
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
    ///     linode-cli linodes clone 123 \
    ///   --linode_id 124 \
    ///   --region us-east \
    ///   --type g6-standard-2 \
    ///   --label cloned-linode \
    ///   --backups_enabled true \
    ///   --placement_group.id 528 \
    ///   --disks 25674 \
    ///   --configs 23456 \
    ///   --private_ip true \
    ///   --metadata.user_data I2Nsb3VkLWNvbmZpZw==
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     linodes:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionLinodeInstancesLinodeIdCloneResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/clone", & request.api_version, & request
                    .linode_id
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
            crate::models::PostApiVersionLinodeInstancesLinodeIdCloneResponse,
        >(response)
            .await
    }
}
