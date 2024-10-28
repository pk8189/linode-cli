#[derive(Clone, Debug)]
pub struct CredentialsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl CredentialsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn reset(
        &self,
    ) -> crate::resources::databases::mysql::instances::credentials::reset::resource_client::ResetClient {
        crate::resources::databases::mysql::instances::credentials::reset::resource_client::ResetClient::new(
            self.base_client.clone(),
        )
    }
    /// __This operation is currently only available for customers who already have an active Managed Database.__
    ///
    /// Display the root username and password for an accessible Managed MySQL Database.
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
    ///     linode-cli databases mysql-creds-view 123
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
        crate::models::GetApiVersionDatabasesMysqlInstancesInstanceIdCredentialsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/databases/mysql/instances/{}/credentials", & request
                    .api_version, & request.instance_id
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
            crate::models::GetApiVersionDatabasesMysqlInstancesInstanceIdCredentialsResponse,
        >(response)
            .await
    }
}
