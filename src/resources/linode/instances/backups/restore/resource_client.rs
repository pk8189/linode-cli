#[derive(Clone, Debug)]
pub struct RestoreClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RestoreClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Restores a Linode's Backup to the specified Linode.
    ///
    /// The following conditions apply:
    ///
    ///   - Backups may not be restored across Regions.
    ///   - Only successfully completed Backups that are not undergoing maintenance can be restored.
    ///   - The Linode that the Backup is being restored to must not itself be in the process of creating a Backup.
    ///
    /// __Note__. When you restore a backup, the restored disk is assigned the same [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) as the original disk. In most cases, this is acceptable and does not cause issues. However, if you attempt to mount both the original disk and the corresponding restore disk at the same time (by assigning them both to devices in your Configuration Profile's __Block Device Assignment__), you will encounter a UUID "collision".
    ///
    /// When this happens, the system selects, and mounts, only one of the disks at random. This is due to both disks sharing the same UUID, and your instance _may fail to boot_ since it will not be clear which disk is root. If your system does boot, you will not see any immediate indication if you are booted into the restored disk or the original disk, and you will be unable to access both disks at the same time.
    ///
    /// To avoid this, we recommend only restoring a backup to the same Compute Instance if you do not intend on mounting them at the same time or are comfortable modifying UUIDs. If you need access to files on both the original disk and the restored disk simultaneously (such as needing to copy files between them), we suggest either restoring the backup to a separate Compute Instance or [creating](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) a new Compute Instance with the desired `backup_id`.
    ///
    /// To learn more about block device assignments and viewing your disks' UUIDs, see our guide on [Configuration Profiles](https://www.linode.com/docs/products/compute/compute-instances/guides/configuration-profiles/#block-device-assignment).
    ///
    /// __Note__. Backups are not encrypted even when they are taken from an encrypted disk. When a backup is restored, and if encryption is enabled, the data stored on the disk is encrypted again.
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
    ///     linode-cli linodes backup-restore 123 123456 \
    ///   --linode_id 234 \
    ///   --overwrite true
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
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/backups/{}/restore", & request.api_version,
                    & request.linode_id, & request.backup_id
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
