#[derive(Clone, Debug)]
pub struct GrantsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl GrantsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns the full grants structure for an account username you specify. This includes all entities on the account, and the level of access this user has to each of them.
    ///
    /// This doesn't apply to the account owner or the current authenticated user. You can run the [List grants](https://techdocs.akamai.com/linode-api/reference/get-profile-grants) operation to view those grants. However, this doesn't show the entities that they _don't_ have access to.
    ///
    /// __Note__. This operation can only be accessed by account users with _unrestricted_ access.
    ///
    ///
    /// <<LB>>
    ///
    /// ---
    ///
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
    ) -> crate::SdkResult<
        crate::models::GetApiVersionAccountUsersUsernameGrantsResponse1000,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/users/{}/grants", & request.api_version, & request
                    .username
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
            crate::models::GetApiVersionAccountUsersUsernameGrantsResponse1000,
        >(response)
            .await
    }
    /// Update the grants a user has. This can be used to give a user access to new entities or actions, or take access away.  You don't need to include the grant for every entity on the account in this request. Any that are not included remain unchanged.
    ///
    /// __Note__. This operation can only be accessed by account users with _unrestricted_ access.
    ///
    /// __Parent and child accounts__
    ///
    /// In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:
    ///
    /// - No child account user can modify the `account_access` grant for the child account parent user (proxy user).
    ///
    /// - An unrestricted child account user can configure all other grants for the proxy user, via `global` object.
    ///
    /// - An unrestricted child account user can enable the `account_access` grant for other child account users. However, enabled child users are still subject to child user restrictions--they can't perform write operations for any billing or account information.
    ///
    ///
    /// <<LB>>
    ///
    /// ---
    ///
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionAccountUsersUsernameGrantsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/users/{}/grants", & request.api_version, & request
                    .username
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
            crate::models::PutApiVersionAccountUsersUsernameGrantsResponse,
        >(response)
            .await
    }
}
