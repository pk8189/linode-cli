#[derive(Clone, Debug)]
pub struct VpcsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl VpcsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn subnets(
        &self,
    ) -> crate::resources::vpcs::subnets::resource_client::SubnetsClient {
        crate::resources::vpcs::subnets::resource_client::SubnetsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn ips(&self) -> crate::resources::vpcs::ips::resource_client::IpsClient {
        crate::resources::vpcs::ips::resource_client::IpsClient::new(
            self.base_client.clone(),
        )
    }
    /// Delete a single VPC and all of its Subnets.
    ///
    /// - The User accessing this operation must have `read_write` grants to the VPC.
    /// - A successful request triggers a `vpc_delete` event and `subnet_delete` events for each deleted VPC Subnet.
    /// - All of the VPC's Subnets must be eligible for deletion. Accordingly, all Configuration Profile Interfaces that each Subnet is assigned to must first be deleted. If those Interfaces are active, the associated Linodes must first be shut down before they can be removed. If any Subnet cannot be deleted, then neither the VPC nor any of its Subnets are deleted.
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
    ///     linode-cli vpcs delete $vpcId
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
            .build_url(&format!("/{}/vpcs/{}", & request.api_version, & request.vpc_id));
        let mut builder = reqwest::Client::default().delete(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<serde_json::Value>(response).await
    }
    /// Display all VPCs on your account.
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
    ///     linode-cli vpcs list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(&format!("/{}/vpcs", & request.api_version));
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
    /// Get information about a single VPC.
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
    ///     linode-cli vpcs view $vpcId
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionVpcsVpcIdResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/vpcs/{}", & request.api_version, & request.vpc_id));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionVpcsVpcIdResponse,
        >(response)
            .await
    }
    /// Create a new VPC and optionally associated VPC Subnets.
    ///
    /// - Users must have the `add_vpc` grant to access this operation.
    /// - A successful request triggers a `vpc_create` event and `subnet_create` events for any created VPC Subnets.
    ///
    /// Once a VPC is created, it can be attached to a Linode by assigning a VPC Subnet to one of the Linode's Configuration Profile Interfaces. This step can be accomplished with the following operations:
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
    ///     linode-cli vpcs create \
    ///   --description "A description of my VPC." \
    ///   --label cool-vpc \
    ///   --region us-east \
    ///   --subnets.label cool-vpc-subnet \
    ///   --subnets.ipv4 10.0.1.0/24
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
    ) -> crate::SdkResult<crate::models::PostApiVersionVpcsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/vpcs", & request.api_version));
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
            crate::models::PostApiVersionVpcsResponse,
        >(response)
            .await
    }
    /// Update an existing VPC.
    ///
    /// - The User accessing this operation must have `read_write` grants to the VPC.
    /// - A successful request triggers a `vpc_update` event.
    ///
    /// To update a VPC's Subnet, run the [Update a VPC subnet](https://techdocs.akamai.com/linode-api/reference/put-vpc-subnet) operation.
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
    ///     linode-cli vpcs update $vpcId \
    ///   --description "A description of my VPC."
    ///   --label cool-vpc
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
    ) -> crate::SdkResult<crate::models::PutApiVersionVpcsVpcIdResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/vpcs/{}", & request.api_version, & request.vpc_id));
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
            crate::models::PutApiVersionVpcsVpcIdResponse,
        >(response)
            .await
    }
}
