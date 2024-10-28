#[derive(Clone, Debug)]
pub struct PlanClient {
    base_client: crate::core::base_client::BaseClient,
}
impl PlanClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Get the details of your current Longview plan. This returns a `LongviewSubscription` object for your current Longview Pro plan, or an empty set `{}` if your current plan is Longview Free.
    ///
    /// You must have at least one of the following `global` [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) in order to run this operation:
    ///
    ///   - `"account_access": read_write`
    ///   - `"account_access": read_only`
    ///   - `"longview_subscription": true`
    ///   - `"add_longview": true`
    ///
    /// To update your subscription plan, send a request to [Update a Longview plan](https://techdocs.akamai.com/linode-api/reference/put-longview-plan).
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
    ///     linode-cli longview plan-view
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     longview:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionLongviewPlanResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/longview/plan", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionLongviewPlanResponse,
        >(response)
            .await
    }
    /// Update your Longview plan to that of the given subscription ID. This returns a `LongviewSubscription` object for the updated Longview Pro plan, or an empty set `{}` if the updated plan is Longview Free.
    ///
    /// You must have `"longview_subscription": true` configured as a `global` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) in order to run this operation.
    ///
    /// You can send a request to the [List Longview subscriptions](https://techdocs.akamai.com/linode-api/reference/get-longview-subscriptions) operation to receive the details, including `id`'s, of each plan.
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
    ///     linode-cli longview plan-update --longview_subscription longview-10
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     longview:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<crate::models::PutApiVersionLongviewPlanResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/longview/plan", & request.api_version));
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
            crate::models::PutApiVersionLongviewPlanResponse,
        >(response)
            .await
    }
}
