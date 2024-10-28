#[derive(Clone, Debug)]
pub struct ChildAccountsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ChildAccountsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn token_resource(
        &self,
    ) -> crate::resources::account::child_accounts::token_resource::resource_client::TokenClient {
        crate::resources::account::child_accounts::token_resource::resource_client::TokenClient::new(
            self.base_client.clone(),
        )
    }
    /// Returns a paginated list of basic information for the child accounts that exist for your parent account. See [Parent and Child Accounts for Akamai Partners](https://www.linode.com/docs/guides/parent-child-accounts/) for details on these accounts.
    ///
    /// __Note__. This operation can only be accessed by an unrestricted parent user, or restricted parent user with the `child_account_access` grant.
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
    ///     linode-cli child-account list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     child_account:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionAccountChildAccountsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account/child-accounts", & request.api_version));
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
            crate::models::GetApiVersionAccountChildAccountsResponse,
        >(response)
            .await
    }
    /// View a specific child account based on its `euuid`. See [Parent and Child Accounts for Akamai Partners](https://www.linode.com/docs/guides/parent-child-accounts/) for details on these accounts.
    ///
    /// __Note__. This operation can only be accessed by an unrestricted user, or restricted user with the `child_account_access` grant.
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
    ///     linode-cli child-account view A1BC2DEF-34GH-567I-J890KLMN12O34P56
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     child_account:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionAccountChildAccountsEuuidResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/child-accounts/{}", & request.api_version, & request
                    .euuid
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
            crate::models::GetApiVersionAccountChildAccountsEuuidResponse,
        >(response)
            .await
    }
}
