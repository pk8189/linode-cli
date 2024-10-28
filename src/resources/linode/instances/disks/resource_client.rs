#[derive(Clone, Debug)]
pub struct DisksClient {
    base_client: crate::core::base_client::BaseClient,
}
impl DisksClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn clone(
        &self,
    ) -> crate::resources::linode::instances::disks::clone::resource_client::CloneClient {
        crate::resources::linode::instances::disks::clone::resource_client::CloneClient::new(
            self.base_client.clone(),
        )
    }
    pub fn password(
        &self,
    ) -> crate::resources::linode::instances::disks::password::resource_client::PasswordClient {
        crate::resources::linode::instances::disks::password::resource_client::PasswordClient::new(
            self.base_client.clone(),
        )
    }
    pub fn resize(
        &self,
    ) -> crate::resources::linode::instances::disks::resize::resource_client::ResizeClient {
        crate::resources::linode::instances::disks::resize::resource_client::ResizeClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a Disk you have permission to `read_write`.
    ///
    /// __Deleting a Disk is a destructive action and cannot be undone.__
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
    ///     linode-cli linodes disk-delete 123 24674
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
    pub async fn delete(
        &self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/disks/{}", & request.api_version, & request
                    .linode_id, & request.disk_id
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
    /// View Disk information for Disks associated with this Linode.
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
    ///     linode-cli linodes disks-list 123
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     linodes:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLinodeInstancesLinodeIdDisksResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/disks", & request.api_version, & request
                    .linode_id
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
            crate::models::GetApiVersionLinodeInstancesLinodeIdDisksResponse,
        >(response)
            .await
    }
    /// View Disk information for a Disk associated with this Linode.
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
    ///     linode-cli linodes disk-view 123 25674
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     linodes:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLinodeInstancesLinodeIdDisksDiskIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/disks/{}", & request.api_version, & request
                    .linode_id, & request.disk_id
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
            crate::models::GetApiVersionLinodeInstancesLinodeIdDisksDiskIdResponse,
        >(response)
            .await
    }
    /// Add a new disk to an existing Linode. You can create an empty disk to manually configure it later. You can also target a stored `image` to build the disk using a pre-configured file system.
    ///
    /// - A Linode can have up to 50 disks.
    ///
    /// - When creating an empty disk, you need to provide a `label` for it. If you don't include a `label`, you need to target an `image` instead.
    ///
    /// - When you create a disk from an `image`, you need to set a `root_pass` for the disk.
    ///
    /// - The default file system for a new disk is `ext4`. If you're creating one from an `image`, the disk inherits the file system of that `image`, is unless you specify otherwise.
    ///
    /// - When you deploy a StackScript on a disk:
    ///
    ///   - You can run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts) to review available StackScripts.
    ///
    ///   - You need to include a compatible `image` when creating the disk. Run [Get a StackScript](https://techdocs.akamai.com/linode-api/reference/get-stack-script) to review compatible images.
    ///
    ///   - You should supply SSH keys for the disk's root user, using the `authorized_keys` field.
    ///
    ///   - You can include individual users via the `authorized_users` field. Before you can add a user, it needs an SSH key assigned to its profile. See [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) for more information.
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
    ///     linode-cli linodes disk-create 123 \
    ///   --size 1300 \
    ///   --authorized_keys "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer" \
    ///   --authorized_users "myUser" \
    ///   --authorized_users "secondaryUser" \
    ///   --root_pass aComplex@Password \
    ///   --image "linode/debian9" \
    ///   --stackscript_id 10079 \
    ///   --stackscript_data '{"gh_username": "linode"}'
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
        crate::models::PostApiVersionLinodeInstancesLinodeIdDisksResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/disks", & request.api_version, & request
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
            crate::models::PostApiVersionLinodeInstancesLinodeIdDisksResponse,
        >(response)
            .await
    }
    /// Updates a Disk that you have permission to `read_write`.
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
    ///     linode-cli linodes disk-update 123 25674 \
    ///   --label "Debian 9 Disk"
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
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionLinodeInstancesLinodeIdDisksDiskIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/disks/{}", & request.api_version, & request
                    .linode_id, & request.disk_id
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
            crate::models::PutApiVersionLinodeInstancesLinodeIdDisksDiskIdResponse,
        >(response)
            .await
    }
}
