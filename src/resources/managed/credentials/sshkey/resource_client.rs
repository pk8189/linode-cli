#[derive(Clone, Debug)]
pub struct SshkeyClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SshkeyClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns the unique SSH public key assigned to your Linode account's Managed service. If you [add this public key](https://www.linode.com/docs/products/services/managed/get-started/#adding-the-public-key) to a Linode on your account, Linode special forces will be able to log in to the Linode with this key when attempting to resolve issues.
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
    ///     linode-cli managed credential-sshkey-view
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionManagedCredentialsSshkeyResponse> {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/managed/credentials/sshkey", & request.api_version),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionManagedCredentialsSshkeyResponse,
        >(response)
            .await
    }
}
