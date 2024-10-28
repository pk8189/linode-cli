#[derive(Clone, Debug)]
pub struct RestoreClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RestoreClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Restore a backup to a Managed MySQL Database on your Account.
    ///
    /// Requires `read_write` access to the Database.
    ///
    /// The Database must have an `active`, `degraded`, or `failed` status to perform this operation.
    ///
    /// __Note__. Restoring from a backup will erase all existing data on the database instance and replace it with backup data.
    ///
    /// __Note__. Currently, restoring a backup after resetting Managed Database credentials results in a failed cluster. Please contact Customer Support if this occurs.
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
    ///     linode-cli databases mysql-backup-restore 123 456
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     databases:read_write
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
                    "/{}/databases/mysql/instances/{}/backups/{}/restore", & request
                    .api_version, & request.instance_id, & request.backup_id
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
