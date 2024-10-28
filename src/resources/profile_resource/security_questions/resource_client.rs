#[derive(Clone, Debug)]
pub struct SecurityQuestionsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SecurityQuestionsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns a collection of security questions and their responses, if any, for your User Profile.
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
    ///     linode-cli security-questions list
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
    ) -> crate::SdkResult<crate::models::GetApiVersionProfileSecurityQuestionsResponse> {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/profile/security-questions", & request.api_version),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionProfileSecurityQuestionsResponse,
        >(response)
            .await
    }
    /// Adds security question responses for your User.
    ///
    /// Requires exactly three unique questions.
    ///
    /// Previous responses are overwritten if answered or reset to `null` if unanswered.
    ///
    /// __Note__. Security questions must be answered for your User prior to accessing the [Create a two factor secret](https://techdocs.akamai.com/linode-api/reference/post-tfa-enable) operation.
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
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionProfileSecurityQuestionsResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/profile/security-questions", & request.api_version),
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
            crate::models::PostApiVersionProfileSecurityQuestionsResponse,
        >(response)
            .await
    }
}
