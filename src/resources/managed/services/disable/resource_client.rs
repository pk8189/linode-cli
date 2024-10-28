#[derive(Clone, Debug)]
pub struct DisableClient {
    base_client: crate::core::base_client::BaseClient,
}
impl DisableClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Temporarily disables monitoring of a Managed Service.
    ///
    /// This operation can only be accessed by the unrestricted users of an account.
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
    ///     linode-cli managed service-disable 9994
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionManagedServicesServiceIdDisableResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/managed/services/{}/disable", & request.api_version, & request
                    .service_id
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostApiVersionManagedServicesServiceIdDisableResponse,
        >(response)
            .await
    }
}
