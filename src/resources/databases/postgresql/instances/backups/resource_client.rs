#[derive(Clone, Debug)]
pub struct BackupsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl BackupsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn restore(
        &self,
    ) -> crate::resources::databases::postgresql::instances::backups::restore::resource_client::RestoreClient {
        crate::resources::databases::postgresql::instances::backups::restore::resource_client::RestoreClient::new(
            self.base_client.clone(),
        )
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Delete a single backup for an accessible Managed PostgreSQL Database.
    ///
    /// Requires `read_write` access to the Database.
    ///
    /// The Database must not be provisioning to perform this operation.
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
    ///     linode-cli databases postgresql-backup-delete 123 456
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
    pub async fn delete(
        &self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/postgresql/instances/{}/backups/{}", & request
                    .api_version, & request.instance_id, & request.backup_id
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
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Display all backups for an accessible Managed PostgreSQL Database.
    ///
    /// The Database must not be provisioning to perform this operation.
    ///
    /// Database `auto` type backups are created every 24 hours at 0:00 UTC. Each `auto` backup is retained for 7 days.
    ///
    /// Database `snapshot` type backups are created by accessing the [Create a managed PostgreSQL database backup snapshot](https://techdocs.akamai.com/linode-api/reference/post-databases-postgre-sql-instance-backup) operation.
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
    ///     linode-cli databases postgresql-backups-list 123
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
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/postgresql/instances/{}/backups", & request
                    .api_version, & request.instance_id
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
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Display information for a single backup for an accessible Managed PostgreSQL Database.
    ///
    /// The Database must not be provisioning to perform this operation.
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
    ///     linode-cli databases postgresql-backup-view 123 456
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
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/postgresql/instances/{}/backups/{}", & request
                    .api_version, & request.instance_id, & request.backup_id
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
            crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdResponse,
        >(response)
            .await
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Creates a snapshot backup of a Managed PostgreSQL Database.
    ///
    /// Requires `read_write` access to the Database.
    ///
    /// Up to 3 snapshot backups for each Database can be stored at a time. If 3 snapshots have been created for a Database, one must be deleted before another can be made.
    ///
    /// Backups generated by this operation have the type `snapshot`. Snapshot backups may take several minutes to complete, after which they will be accessible to view or restore.
    ///
    /// The Database must have an `active` status to perform this operation. If another backup is in progress, it must complete before a new backup can be initiated.
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
    ///     linode-cli databases postgresql-backup-snapshot 123 \
    ///   --label snapshot1 \
    ///   --target primary
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
                    "/{}/databases/postgresql/instances/{}/backups", & request
                    .api_version, & request.instance_id
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
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
}
