#[derive(Clone, Debug)]
pub struct InstancesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl InstancesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn configs(
        &self,
    ) -> crate::resources::linode::instances::configs::resource_client::ConfigsClient {
        crate::resources::linode::instances::configs::resource_client::ConfigsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn disks(
        &self,
    ) -> crate::resources::linode::instances::disks::resource_client::DisksClient {
        crate::resources::linode::instances::disks::resource_client::DisksClient::new(
            self.base_client.clone(),
        )
    }
    pub fn ips(
        &self,
    ) -> crate::resources::linode::instances::ips::resource_client::IpsClient {
        crate::resources::linode::instances::ips::resource_client::IpsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn backups(
        &self,
    ) -> crate::resources::linode::instances::backups::resource_client::BackupsClient {
        crate::resources::linode::instances::backups::resource_client::BackupsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn firewalls(
        &self,
    ) -> crate::resources::linode::instances::firewalls::resource_client::FirewallsClient {
        crate::resources::linode::instances::firewalls::resource_client::FirewallsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn nodebalancers(
        &self,
    ) -> crate::resources::linode::instances::nodebalancers::resource_client::NodebalancersClient {
        crate::resources::linode::instances::nodebalancers::resource_client::NodebalancersClient::new(
            self.base_client.clone(),
        )
    }
    pub fn stats(
        &self,
    ) -> crate::resources::linode::instances::stats::resource_client::StatsClient {
        crate::resources::linode::instances::stats::resource_client::StatsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn transfer(
        &self,
    ) -> crate::resources::linode::instances::transfer::resource_client::TransferClient {
        crate::resources::linode::instances::transfer::resource_client::TransferClient::new(
            self.base_client.clone(),
        )
    }
    pub fn volumes(
        &self,
    ) -> crate::resources::linode::instances::volumes::resource_client::VolumesClient {
        crate::resources::linode::instances::volumes::resource_client::VolumesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn boot(
        &self,
    ) -> crate::resources::linode::instances::boot::resource_client::BootClient {
        crate::resources::linode::instances::boot::resource_client::BootClient::new(
            self.base_client.clone(),
        )
    }
    pub fn clone(
        &self,
    ) -> crate::resources::linode::instances::clone::resource_client::CloneClient {
        crate::resources::linode::instances::clone::resource_client::CloneClient::new(
            self.base_client.clone(),
        )
    }
    pub fn migrate(
        &self,
    ) -> crate::resources::linode::instances::migrate::resource_client::MigrateClient {
        crate::resources::linode::instances::migrate::resource_client::MigrateClient::new(
            self.base_client.clone(),
        )
    }
    pub fn mutate(
        &self,
    ) -> crate::resources::linode::instances::mutate::resource_client::MutateClient {
        crate::resources::linode::instances::mutate::resource_client::MutateClient::new(
            self.base_client.clone(),
        )
    }
    pub fn password(
        &self,
    ) -> crate::resources::linode::instances::password::resource_client::PasswordClient {
        crate::resources::linode::instances::password::resource_client::PasswordClient::new(
            self.base_client.clone(),
        )
    }
    pub fn reboot(
        &self,
    ) -> crate::resources::linode::instances::reboot::resource_client::RebootClient {
        crate::resources::linode::instances::reboot::resource_client::RebootClient::new(
            self.base_client.clone(),
        )
    }
    pub fn rebuild(
        &self,
    ) -> crate::resources::linode::instances::rebuild::resource_client::RebuildClient {
        crate::resources::linode::instances::rebuild::resource_client::RebuildClient::new(
            self.base_client.clone(),
        )
    }
    pub fn rescue(
        &self,
    ) -> crate::resources::linode::instances::rescue::resource_client::RescueClient {
        crate::resources::linode::instances::rescue::resource_client::RescueClient::new(
            self.base_client.clone(),
        )
    }
    pub fn resize(
        &self,
    ) -> crate::resources::linode::instances::resize::resource_client::ResizeClient {
        crate::resources::linode::instances::resize::resource_client::ResizeClient::new(
            self.base_client.clone(),
        )
    }
    pub fn shutdown(
        &self,
    ) -> crate::resources::linode::instances::shutdown::resource_client::ShutdownClient {
        crate::resources::linode::instances::shutdown::resource_client::ShutdownClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a Linode you have permission to `read_write`.
    ///
    /// __Deleting a Linode is a destructive action and cannot be undone.__
    ///
    /// Additionally, deleting a Linode:
    ///
    ///   - Gives up any IP addresses the Linode was assigned.
    ///   - Deletes all Disks, Backups, Configs, etc.
    ///   - Detaches any Volumes associated with the Linode.
    ///   - Stops billing for the Linode and its associated services. You will be billed for time used within the billing period the Linode was active.
    ///
    /// Linodes that are in the process of [cloning](https://techdocs.akamai.com/linode-api/reference/post-clone-linode-instance) or [backup restoration](https://techdocs.akamai.com/linode-api/reference/post-restore-backup) cannot be deleted.
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
    ///     linode-cli linodes delete 123
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
                    "/{}/linode/instances/{}", & request.api_version, & request.linode_id
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
    /// Returns a paginated list of Linodes you have permission to view.
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
    ///     linode-cli linodes list
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
    ) -> crate::SdkResult<crate::models::GetApiVersionLinodeInstancesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/linode/instances", & request.api_version));
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
            crate::models::GetApiVersionLinodeInstancesResponse,
        >(response)
            .await
    }
    /// Get a specific Linode by ID.
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
    ///     linode-cli linodes view 123
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
    ) -> crate::SdkResult<crate::models::GetApiVersionLinodeInstancesLinodeIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}", & request.api_version, & request.linode_id
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
            crate::models::GetApiVersionLinodeInstancesLinodeIdResponse,
        >(response)
            .await
    }
    /// Creates a Linode Instance on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Creating a new Linode will incur a charge on your Account.
    ///
    /// Linodes can be created using one of the available Types. Run [List Linode types](https://techdocs.akamai.com/linode-api/reference/get-linode-types) to get more information about each Type's specs and cost.
    ///
    /// Linodes can be created in any one of our available Regions, which are accessible from the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation.
    ///
    /// In an effort to fight spam, Linode restricts outbound connections on ports 25, 465, and 587 on all Linodes for new accounts created after November 5th, 2019. For more information, see our guide on [Running a Mail Server](https://www.linode.com/docs/guides/running-a-mail-server/).
    ///
    /// __Important__. You must be an unrestricted User in order to add or modify tags on Linodes.
    ///
    /// Linodes can be created in a number of ways:
    ///
    /// - Using a Linode Public Image distribution or a Private Image you created based on another Linode.
    ///
    ///   - Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation with authentication to view all available Images.
    ///
    ///   - The Linode will be `running` after it completes `provisioning`.
    ///   - A default config with two Disks, one being a 512 swap disk, is created.
    ///     - `swap_size` can be used to customize the swap disk size.
    ///   - Requires a `root_pass` be supplied to use for the root User's Account.
    ///   - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.
    ///   - You may also supply a list of usernames via the `authorized_users` field.
    ///     - These users must have an SSH Key associated with your Profile first. See the [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key)) operation for more information.
    ///
    /// - Using cloud-init with [Metadata](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata/).
    ///   - Automate system configuration and software installation by providing a base-64 encoded [cloud-config](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata-cloud-config/) file.
    ///   - Requires a compatible Image. You can determine compatible Images by checking for `cloud-init` under `capabilities` when running [List images](https://techdocs.akamai.com/linode-api/reference/get-images).
    ///   - Requires a compatible Region.  You can determine compatible Regions by checking for `Metadata` under `capabilities` when running [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions).
    ///
    /// - Using a StackScript.
    ///
    ///   - Run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts) for a list of available StackScripts.
    ///   - The Linode will be `running` after it completes `provisioning`.
    ///   - Requires a compatible Image to be supplied.
    ///     - Run [Get a StackScript](https://techdocs.akamai.com/linode-api/reference/get-stack-script) for compatible Images.
    ///   - Requires a `root_pass` be supplied to use for the root User's Account.
    ///   - It is recommended to supply SSH keys for the root User using the `authorized_keys` field.
    ///   - You may also supply a list of usernames via the `authorized_users` field.
    ///     - These users must have an SSH Key associated with your Profile first. See [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) for more information.
    ///
    /// - Using one of your other Linode's backups.
    ///   - You must create a Linode large enough to accommodate the Backup's size.
    ///   - The Disks and Config will match that of the Linode that was backed up.
    ///   - The `root_pass` will match that of the Linode that was backed up.
    ///
    /// - Attached to a private VLAN.
    ///   - Review the `interfaces` property of the [Request Body Schema](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) for details.
    ///   - For more information, see our guide on [Getting Started with VLANs](https://www.linode.com/docs/products/networking/vlans/get-started/).
    ///
    /// - Create an empty Linode.
    ///   - The Linode will remain `offline` and must be manually started.
    ///     - Run [Boot a Linode](https://techdocs.akamai.com/linode-api/reference/post-boot-linode-instance).
    ///   - Disks and Configs must be created manually.
    ///   - This is only recommended for advanced use cases.
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
    ///     linode-cli linodes create \
    ///   --label linode123 \
    ///   --root_pass aComplex@Password \
    ///   --booted true \
    ///   --stackscript_id 10079 \
    ///   --stackscript_data '{"gh_username": "linode"}' \
    ///   --region us-east \
    ///   --disk_encryption enabled\
    ///   --placement_group.id 528 \
    ///   --type g6-standard-2 \
    ///   --authorized_keys "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer" \
    ///   --authorized_users "myUser" \
    ///   --authorized_users "secondaryUser" \
    ///   --metadata.user_data "I2Nsb3VkLWNvbmZpZw==" \
    ///   --firewall_id 9000
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
    ) -> crate::SdkResult<crate::models::PostApiVersionLinodeInstancesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/linode/instances", & request.api_version));
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
            crate::models::PostApiVersionLinodeInstancesResponse,
        >(response)
            .await
    }
    /// Updates a Linode that you have permission to `read_write`.
    ///
    /// __Important__. You must be an unrestricted User in order to add or modify tags on Linodes.
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
    ///     linode-cli linodes update 7833080 \
    ///   --label linode123 \
    ///   --backups.schedule.day "Saturday" \
    ///   --backups.schedule.window "W22" \
    ///   --alerts.cpu 180 \
    ///   --alerts.network_in 10 \
    ///   --alerts.network_out 10 \
    ///   --alerts.transfer_quota 80 \
    ///   --alerts.io 10000
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
    ) -> crate::SdkResult<crate::models::PutApiVersionLinodeInstancesLinodeIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}", & request.api_version, & request.linode_id
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
            crate::models::PutApiVersionLinodeInstancesLinodeIdResponse,
        >(response)
            .await
    }
}
