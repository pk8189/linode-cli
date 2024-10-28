#[derive(Clone, Debug)]
pub struct SubnetsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SubnetsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Delete a single VPC Subnet.
    ///
    /// The user accessing this operation must have `read_write` grants to the VPC. A successful request triggers a `subnet_delete` event.
    ///
    /// __Note__. You need to delete all the Configuration Profile Interfaces that this Subnet is assigned to before you can delete it. If those Interfaces are active, the associated Linode needs to be shut down before they can be removed.
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
    ///     linode-cli vpcs subnet-delete $vpcId $vpcSubnetId
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     vpc:read_write
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
                    "/{}/vpcs/{}/subnets/{}", & request.api_version, & request.vpc_id, &
                    request.vpc_subnet_id
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
    /// Get information about all VPC Subnets associated with a VPC.
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
    ///     linode-cli vpcs subnets-list $vpcId
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/vpcs/{}/subnets", & request.api_version, & request.vpc_id),
            );
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
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
    /// Get information about a single VPC Subnet.
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
    ///     linode-cli vpcs subnet-view $vpcId $vpcSubnetId
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionVpcsVpcIdSubnetsVpcSubnetIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/vpcs/{}/subnets/{}", & request.api_version, & request.vpc_id, &
                    request.vpc_subnet_id
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
            crate::models::GetApiVersionVpcsVpcIdSubnetsVpcSubnetIdResponse,
        >(response)
            .await
    }
    /// Create a VPC Subnet.
    ///
    /// - The User accessing this operation must have `read_write` grants to the VPC.
    /// - A successful request triggers a `subnet_create` event.
    ///
    /// Once a VPC Subnet is created, it can be attached to a Linode by assigning the Subnet to one of the Linode's Configuration Profile Interfaces. This step can be accomplished with the following operations:
    ///
    /// - [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance)
    /// - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config)
    /// - [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config)
    /// - [Add a configuration profile interface](https://techdocs.akamai.com/linode-api/reference/post-linode-config-interface)
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
    ///     linode-cli vpcs subnet-create $vpcId \
    ///   --label cool-vpc-subnet \
    ///   --ipv4 10.0.1.0/24
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     vpc:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostApiVersionVpcsVpcIdSubnetsResponse> {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/vpcs/{}/subnets", & request.api_version, & request.vpc_id),
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
            crate::models::PostApiVersionVpcsVpcIdSubnetsResponse,
        >(response)
            .await
    }
    /// Update a VPC Subnet.
    ///
    /// - The User accessing this operation must have `read_write` grants to the VPC.
    /// - A successful request triggers a `subnet_update` event.
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
    ///     linode-cli vpcs subnet-update $vpcId \
    ///   --label cool-vpc-subnet
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     vpc:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/vpcs/{}/subnets/{}", & request.api_version, & request.vpc_id, &
                    request.vpc_subnet_id
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
            crate::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdResponse,
        >(response)
            .await
    }
}
