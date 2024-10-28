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
    ) -> crate::resources::databases::postgresql::instances::backups::resource_client::BackupsClient {
        crate::resources::databases::postgresql::instances::backups::resource_client::BackupsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn credentials(
        &self,
    ) -> crate::resources::databases::postgresql::instances::credentials::resource_client::CredentialsClient {
        crate::resources::databases::postgresql::instances::credentials::resource_client::CredentialsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn ssl_resource(
        &self,
    ) -> crate::resources::databases::postgresql::instances::ssl_resource::resource_client::SslClient {
        crate::resources::databases::postgresql::instances::ssl_resource::resource_client::SslClient::new(
            self.base_client.clone(),
        )
    }
    pub fn patch(
        &self,
    ) -> crate::resources::databases::postgresql::instances::patch::resource_client::PatchClient {
        crate::resources::databases::postgresql::instances::patch::resource_client::PatchClient::new(
            self.base_client.clone(),
        )
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Remove a Managed PostgreSQL Database from your Account.
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
    ///     linode-cli databases postgresql-delete 123
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
                    "/{}/databases/postgresql/instances/{}", & request.api_version, &
                    request.instance_id
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
    /// Display all accessible Managed PostgreSQL Databases.
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
    ///     linode-cli databases postgresql-list
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
            .build_url(
                &format!("/{}/databases/postgresql/instances", & request.api_version),
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
    /// Display information for a single, accessible Managed PostgreSQL Database.
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
    ///     linode-cli databases postgresql-view 123
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
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/postgresql/instances/{}", & request.api_version, &
                    request.instance_id
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
            crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdResponse,
        >(response)
            .await
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Provision a Managed PostgreSQL Database.
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
    /// All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed PostgreSQL Database during configurable maintenance windows.
    ///
    /// - If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.
    ///
    /// - __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then migrate your databases from the original Managed Database cluster to the new one.
    ///
    /// - To modify update the maintenance window for a Database, run the [Update a managed PostgreSQL database](https://techdocs.akamai.com/linode-api/reference/put-databases-postgre-sql-instance) operation.
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
    ///     linode-cli databases postgresql-create \
    ///   --label example-db \
    ///   --region us-east \
    ///   --type g6-dedicated-2 \
    ///   --cluster_size 3 \
    ///   --engine postgresql/13.2 \
    ///   --encrypted false \
    ///   --ssl_connection false \
    ///   --replication_type asynch \
    ///   --replication_commit_type local \
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
    ) -> crate::SdkResult<
        crate::models::PostApiVersionDatabasesPostgresqlInstancesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/databases/postgresql/instances", & request.api_version),
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
            crate::models::PostApiVersionDatabasesPostgresqlInstancesResponse,
        >(response)
            .await
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Update a Managed PostgreSQL Database.
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
    /// All Managed Databases include automatic patch updates, which apply security patches and updates to the underlying operating system of the Managed PostgreSQL Database. The maintenance window for these updates is configured with the Managed Database's `updates` property.
    ///
    /// - If your database cluster is configured with a single node, you will experience downtime during this maintenance window when any updates occur. It's recommended that you adjust this window to match a time that will be the least disruptive for your application and users. You may also want to consider upgrading to a high availability plan to avoid any downtime due to maintenance.
    ///
    /// - __The database software is not updated automatically.__ To upgrade to a new database engine version, consider deploying a new Managed Database with your preferred version. You can then migrate your databases from the original Managed Database cluster to the new one.
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
    ///     linode-cli databases postgresql-update 123 \
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
        crate::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/postgresql/instances/{}", & request.api_version, &
                    request.instance_id
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
            crate::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdResponse,
        >(response)
            .await
    }
}
