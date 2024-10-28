#[derive(Clone, Debug)]
pub struct CloneClient {
    base_client: crate::core::base_client::BaseClient,
}
impl CloneClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Copies a disk, byte-for-byte, into a new disk on the same Linode. The operation fails if the target doesn't have enough storage space. A Linode can have up to 50 disks.
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
    ///     linode-cli linodes disk-clone
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
        crate::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/disks/{}/clone", & request.api_version, &
                    request.linode_id, & request.disk_id
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneResponse,
        >(response)
            .await
    }
}
