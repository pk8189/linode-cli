#[derive(Clone, Debug)]
pub struct RebuildClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RebuildClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Rebuilds a Linode you have the `read_write` permission to modify.
    ///
    /// A rebuild will first shut down the Linode, delete all disks and configs on the Linode, and then deploy a new `image` to the Linode with the given attributes. Additionally:
    ///
    ///   - Requires an `image` be supplied.
    ///   - Requires a `root_pass` be supplied to use for the root User's Account.
    ///   - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.
    ///   - Linodes utilizing Metadata (`"has_user_data": true`) should include `metadata.user_data` in the rebuild request to continue using the service.
    ///
    /// During a rebuild, you can `enable` or `disable` local disk encryption. If disk encryption is not included in the request, the previous `disk_encryption` value is used. Disk encryption cannot be disabled if the compute instance is attached to a LKE nodepool.
    ///
    /// You also have the option to resize the Linode to a different plan by including the `type` parameter with your request. Note that resizing involves migrating the Linode to a new hardware host, while rebuilding without resizing maintains the same hardware host. Resizing also requires significantly more time for completion of this operation. The following additional conditions apply:
    ///
    ///   - The Linode must not have a pending migration.
    ///   - Your Account cannot have an outstanding balance.
    ///   - The Linode must not have more disk allocation than the new Type allows.
    ///     - In that situation, you must first delete or resize the disk to be smaller.
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
    ///     linode-cli linodes rebuild 123 \
    ///   --image "linode/debian9" \
    ///   --root_pass aComplex@Password \
    ///   --disk_encryption disabled \
    ///   --authorized_keys "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer" \
    ///   --authorized_users "myUsername" \
    ///   --authorized_users "secondaryUsername" \
    ///   --booted true \
    ///   --stackscript_id 10079 \
    ///   --stackscript_data '{"gh_username": "linode"}' \
    ///   --type "g6-standard-2" \
    ///   --metadata.userdata "I2Nsb3VkLWNvbmZpZw=="
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
        crate::models::PostApiVersionLinodeInstancesLinodeIdRebuildResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/rebuild", & request.api_version, & request
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
            crate::models::PostApiVersionLinodeInstancesLinodeIdRebuildResponse,
        >(response)
            .await
    }
}
