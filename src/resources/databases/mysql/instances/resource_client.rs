#[derive(Clone, Debug)]
pub struct InstancesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl InstancesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn backups(
        &self,
    ) -> crate::resources::databases::mysql::instances::backups::resource_client::BackupsClient {
        crate::resources::databases::mysql::instances::backups::resource_client::BackupsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn credentials(
        &self,
    ) -> crate::resources::databases::mysql::instances::credentials::resource_client::CredentialsClient {
        crate::resources::databases::mysql::instances::credentials::resource_client::CredentialsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn ssl_resource(
        &self,
    ) -> crate::resources::databases::mysql::instances::ssl_resource::resource_client::SslClient {
        crate::resources::databases::mysql::instances::ssl_resource::resource_client::SslClient::new(
            self.base_client.clone(),
        )
    }
    pub fn patch(
        &self,
    ) -> crate::resources::databases::mysql::instances::patch::resource_client::PatchClient {
        crate::resources::databases::mysql::instances::patch::resource_client::PatchClient::new(
            self.base_client.clone(),
        )
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Remove a Managed MySQL Database from your Account.
    ///
    /// Requires `read_write` access to the Database.
    ///
    /// The Database must have an `active`, `failed`, or `degraded` status to perform this operation.
    ///
    /// Only unrestricted Users can access this operation, and have access regardless of the acting token's OAuth scopes.
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
    ///     linode-cli databases mysql-delete 123
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
                    "/{}/databases/mysql/instances/{}", & request.api_version, & request
                    .instance_id
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
    /// Display all accessible Managed MySQL Databases.
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
    ///     linode-cli databases mysql-list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     databases:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(&format!("/{}/databases/mysql/instances", & request.api_version));
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
    /// Display information for a single, accessible Managed MySQL Database.
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
    ///     linode-cli databases mysql-view 123
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     databases:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionDatabasesMysqlInstancesInstanceIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/mysql/instances/{}", & request.api_version, & request
                    .instance_id
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
            crate::models::GetApiVersionDatabasesMysqlInstancesInstanceIdResponse,
        >(response)
            .await
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Provision a Managed MySQL Database.
    ///
    /// Restricted Users must have the `add_databases` grant to use this operation.
    ///
    /// New instances can take approximately 15 to 30 minutes to provision.
    ///
    /// The `allow_list` is used to control access to the Managed Database.
    ///
    /// - IP addresses and ranges in this list can access the Managed Database. All other sources are blocked.
    ///
    /// - If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.
    ///
    /// - Entering an empty array (`[]`) blocks all connections (both public and private) to the Managed Database.
    ///
    /// All Managed Databases include automatic, daily backups. Up to seven backups are automatically stored for each Managed Database, providing restore points for each day of the past week.
    ///
    /// All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed MySQL Database during configurable maintenance windows.
    ///
    /// - If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.
    ///
    /// - __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then [migrate your databases](https://www.linode.com/docs/products/databases/managed-databases/guides/migrate-mysql/) from the original Managed Database cluster to the new one.
    ///
    /// - To modify update the maintenance window for a Database, run the [Update a managed MySQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-mysql-instance) operation.
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
    ///     linode-cli databases mysql-create \
    ///   --label example-db1 \
    ///   --region us-east \
    ///   --type g6-dedicated-2 \
    ///   --cluster_size 3 \
    ///   --engine mysql/8.0.26 \
    ///   --encrypted false \
    ///   --ssl_connection false \
    ///   --replication_type semi_synch \
    ///   --allow_list 203.0.113.1 \
    ///   --allow_list 192.0.1.0/24
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
    ) -> crate::SdkResult<crate::models::PostApiVersionDatabasesMysqlInstancesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/databases/mysql/instances", & request.api_version));
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
            crate::models::PostApiVersionDatabasesMysqlInstancesResponse,
        >(response)
            .await
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Update a Managed MySQL Database.
    ///
    /// Requires `read_write` access to the Database.
    ///
    /// The Database must have an `active` status to perform this operation.
    ///
    /// Updating addresses in the `allow_list` overwrites any existing addresses.
    ///
    /// - IP addresses and ranges in this list can access the Managed Database. All other sources are blocked.
    ///
    /// - If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.
    ///
    /// - Entering an empty array (`[]`) blocks all connections (both public and private) to the Managed Database.
    ///
    /// - __Note__. Updates to the `allow_list` may take a short period of time to complete, making this operation inappropriate for rapid successive updates to this property.
    ///
    /// All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed MySQL Database. The maintenance window for these updates is configured with the Managed Database's `updates` property.
    ///
    /// - If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.
    ///
    /// - __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then [migrate your databases](https://www.linode.com/docs/products/databases/managed-databases/guides/migrate-mysql/) from the original Managed Database cluster to the new one.
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
    ///     linode-cli databases mysql-update 123 \
    ///   --label example-db \
    ///   --allow_list 203.0.113.1 \
    ///   --allow_list 192.0.1.0/24 \
    ///   --type g6-standard-1 \
    ///   --updates.frequency monthly \
    ///   --updates.duration 3 \
    ///   --updates.hour_of_day 12 \
    ///   --updates.day_of_week 4 \
    ///   --updates.week_of_month 3
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
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionDatabasesMysqlInstancesInstanceIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/mysql/instances/{}", & request.api_version, & request
                    .instance_id
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
            crate::models::PutApiVersionDatabasesMysqlInstancesInstanceIdResponse,
        >(response)
            .await
    }
}
