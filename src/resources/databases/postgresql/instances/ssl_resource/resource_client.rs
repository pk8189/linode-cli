#[derive(Clone, Debug)]
pub struct SslClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SslClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Display the SSL CA certificate for an accessible Managed PostgreSQL Database.
    ///
    /// The Database must have an `active` status to perform this operation.
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
    ///     linode-cli databases postgresql-ssl-cert 123
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
    ) -> crate::SdkResult<
        crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdSslResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/postgresql/instances/{}/ssl", & request.api_version, &
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
            crate::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdSslResponse,
        >(response)
            .await
    }
}
