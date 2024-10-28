#[derive(Clone, Debug)]
pub struct TokenClient {
    base_client: crate::core::base_client::BaseClient,
}
impl TokenClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Create a short-lived bearer token for a parent user on a child account, using the `euuid` of that child account. In the context of the API, a parent user on a child account is referred to as a "proxy user." When Akamai provisions your parent-child account environment, a proxy user is automatically set in the child account. It follows a specific naming convention:
    ///
    ///     <Parent account `company` name>_<SHA256 hash of parent `company` name and child account `euuid`>
    ///
    /// __Note__. The variables above use only the first 15 and 16 characters of these values, respectively.
    ///
    /// The token lets a parent account run API operations through the proxy user, as if they are a child user in the child account.
    ///
    /// These points apply to the use of this operation:
    ///
    /// - To create a token, a parent account user needs the `child_account_access` grant. This lets them use the proxy user on the child account. You can run [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) on a parent account user to check its `child_account_access` setting. To add this access, you can [update](https://techdocs.akamai.com/linode-api/reference/put-user-grants) the parent account user.
    ///
    /// - The created token inherits the permissions of the proxy user. It will never have less.
    ///
    /// - The API returns the raw token in the response. You can't get it again, so be sure to store it.
    ///
    /// Example workflow:
    ///
    /// 1. [List child accounts](https://techdocs.akamai.com/linode-api/reference/get-child-accounts) and store the `euuid` for the applicable one.
    /// 2. Run this operation and store the `token` that's created for the proxy user.
    /// 3. As a parent account user with access to the proxy user in the child account, use this `token` to authenticate API operations, as if you were a child user.
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
    ///     linode-cli child-account create A1BC2DEF-34GH-567I-J890KLMN12O34P56
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     child_account:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionAccountChildAccountsEuuidTokenResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/child-accounts/{}/token", & request.api_version, &
                    request.euuid
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
            crate::models::PostApiVersionAccountChildAccountsEuuidTokenResponse,
        >(response)
            .await
    }
}
