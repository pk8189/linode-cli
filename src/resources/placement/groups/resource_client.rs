#[derive(Clone, Debug)]
pub struct GroupsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl GroupsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn assign(
        &self,
    ) -> crate::resources::placement::groups::assign::resource_client::AssignClient {
        crate::resources::placement::groups::assign::resource_client::AssignClient::new(
            self.base_client.clone(),
        )
    }
    pub fn unassign(
        &self,
    ) -> crate::resources::placement::groups::unassign::resource_client::UnassignClient {
        crate::resources::placement::groups::unassign::resource_client::UnassignClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a placement group you have permission to `read_write`.
    ///
    /// - Deleting a placement group can't be undone.
    /// - All compute instances need to be [removed](https://techdocs.akamai.com/linode-api/reference/post-group-unassign) before you can delete a placement group.
    /// - If your placement group is non-compliant, you can't delete it. You need to wait for our help to bring it [compliant](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#non-compliance-precedence).
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
    ///     linode-cli placement group-delete 528
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
    pub async fn delete(
        &self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/placement/groups/{}", & request.api_version, & request.group_id
                ),
            );
        let mut builder = reqwest::Client::default().delete(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
    /// Returns a paginated list of placement groups you have permission to view.
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
    ///     linode-cli placement groups-list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     placement:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionPlacementGroupsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/placement/groups", & request.api_version));
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
            crate::models::GetApiVersionPlacementGroupsResponse,
        >(response)
            .await
    }
    /// View a specific placement group by ID.
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
    ///     linode-cli placement group-view 528
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     linodes:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionPlacementGroupsGroupIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/placement/groups/{}", & request.api_version, & request.group_id
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
            crate::models::GetApiVersionPlacementGroupsGroupIdResponse,
        >(response)
            .await
    }
    /// Creates a placement group on your account. Placement groups disperse your compute instances across physical machines (hosts) in one of our compute regions. Depending on your workload requirements, you may need your compute instances to follow specific strategies:
    ///
    /// - __Grouped together__. You may want them placed close together to reduce latency between compute instances that are used for an application or workload.
    /// - __Spread apart__. You may want to disperse them across several hosts to support high availability, for example when required for failover.
    ///
    /// <<LB>>
    ///
    /// > 📘
    /// >
    /// > - For this request to complete successfully, your user needs to have the `add_placement` grant.
    /// > - We offer an [example API workflow](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#create-a-placement-group) you can follow to create a placement group.
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
    ///     linode-cli placement group-create \
    ///   --label PG_Miami_failover \
    ///   --region us-mia \
    ///   --placement_group_type "anti-affinity:local" \
    ///   --placement_group_policy strict
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
    ) -> crate::SdkResult<crate::models::PostApiVersionPlacementGroupsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/placement/groups", & request.api_version));
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
            crate::models::PostApiVersionPlacementGroupsResponse,
        >(response)
            .await
    }
    /// Change the `label` for a specific placement group. This is the only value you can update. However, you can [add](https://techdocs.akamai.com/linode-api/reference/post-group-add-linode) more compute instances or [remove](https://techdocs.akamai.com/linode-api/reference/post-group-unassign) existing ones.
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
    ///     linode-cli placement group-update 528 \
    ///   --label PG_Miami_failover_update
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
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<crate::models::PutApiVersionPlacementGroupsGroupIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/placement/groups/{}", & request.api_version, & request.group_id
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
            crate::models::PutApiVersionPlacementGroupsGroupIdResponse,
        >(response)
            .await
    }
}
