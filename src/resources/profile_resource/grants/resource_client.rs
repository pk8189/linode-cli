#[derive(Clone, Debug)]
pub struct GrantsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl GrantsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// This returns a GrantsResponse describing what the acting User has been granted access to.  For unrestricted users, this will return a  204 and no body because unrestricted users have access to everything without grants.  This will not return information about entities you do not have access to.  This operation is useful when writing third-party OAuth applications to see what options you should present to the acting User.
    ///
    /// For example, if they do not have `global.add_linodes`, you might not display a button to deploy a new Linode.
    ///
    /// Any client may run this operation; no OAuth scopes are required.
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionProfileGrantsResponse1000> {
        let url = self
            .base_client
            .build_url(&format!("/{}/profile/grants", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionProfileGrantsResponse1000,
        >(response)
            .await
    }
}
