#[derive(Clone, Debug)]
pub struct UnassignClient {
    base_client: crate::core::base_client::BaseClient,
}
impl UnassignClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Remove one or more compute instances from your placement group.
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
    ///     linode-cli placement unassign-linode 528 \
    ///   --linode 123 456
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     linodes:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionPlacementGroupsGroupIdUnassignResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/placement/groups/{}/unassign", & request.api_version, & request
                    .group_id
                ),
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
            crate::models::PostApiVersionPlacementGroupsGroupIdUnassignResponse,
        >(response)
            .await
    }
}
