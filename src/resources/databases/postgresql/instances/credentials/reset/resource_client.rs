#[derive(Clone, Debug)]
pub struct ResetClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ResetClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Reset the root password for a Managed PostgreSQL Database.
    ///
    /// Requires `read_write` access to the Database.
    ///
    /// A new root password is randomly generated and accessible with the [Get managed PostgreSQL database credentials](https://techdocs.akamai.com/linode-api/reference/get-databases-postgre-sql-instance-credentials) operation.
    ///
    /// Only unrestricted Users can access this operation, and have access regardless of the acting token's OAuth scopes.
    ///
    /// __Note__. It may take several seconds for credentials to reset.
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
    ///     linode-cli databases postgresql-creds-reset 123
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
                    "/{}/databases/postgresql/instances/{}/credentials/reset", & request
                    .api_version, & request.instance_id
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
