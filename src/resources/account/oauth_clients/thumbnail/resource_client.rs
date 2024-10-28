#[derive(Clone, Debug)]
pub struct ThumbnailClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ThumbnailClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns the PNG thumbnail for this OAuth Client.  This is a publicly viewable endpoint, and can be accessed without authentication.
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::BinaryResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/oauth-clients/{}/thumbnail", & request.api_version, &
                    request.client_id
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        Ok(crate::BinaryResponse::new(response).await)
    }
    /// Upload a thumbnail for a client you own.  You must upload a PNG image file that will be returned when the thumbnail is retrieved.  This image will be publicly viewable.
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
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/account/oauth-clients/{}/thumbnail", & request.api_version, &
                    request.client_id
                ),
            );
        let mut builder = reqwest::Client::default().put(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "image/png");
        builder = builder.body(request.data.content.to_vec());
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
}
