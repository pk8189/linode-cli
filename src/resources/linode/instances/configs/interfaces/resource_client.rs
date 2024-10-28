#[derive(Clone, Debug)]
pub struct InterfacesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl InterfacesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn order(
        &self,
    ) -> crate::resources::linode::instances::configs::interfaces::order::resource_client::OrderClient {
        crate::resources::linode::instances::configs::interfaces::order::resource_client::OrderClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes an Interface from the Configuration Profile.
    ///
    /// - The User accessing this operation must have `read_write` grants to the Linode.
    /// - A successful request triggers a `linode_config_update` event.
    /// - Active Interfaces cannot be deleted. The associated Linode must first be shut down (or restarted using another Configuration Profile) before such Interfaces can be deleted from a Configuration Profile.
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
    ///     linode-cli linodes config-interface-delete $linodeId $configId $interfaceId
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
                    "/{}/linode/instances/{}/configs/{}/interfaces/{}", & request
                    .api_version, & request.linode_id, & request.config_id, & request
                    .interface_id
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
    /// Returns an ordered array of all Interfaces associated with this Configuration Profile.
    ///
    /// - The User accessing this operation must have at least `read_only` grants to the Linode.
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
    ///     linode-cli linodes config-interfaces-list $linodeId $configId
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
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<
        Vec<
            crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesResponseItem,
        >,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/configs/{}/interfaces", & request
                    .api_version, & request.linode_id, & request.config_id
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
            Vec<
                crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesResponseItem,
            >,
        >(response)
            .await
    }
    /// Returns a single Configuration Profile Interface.
    ///
    /// - The User accessing this operation must have at least `read_only` grants to the Linode.
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
    ///     linode-cli linodes config-interface-view $linodeId $configId $interfaceId
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
    ) -> crate::SdkResult<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/configs/{}/interfaces/{}", & request
                    .api_version, & request.linode_id, & request.config_id, & request
                    .interface_id
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
            crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponse,
        >(response)
            .await
    }
    /// Creates and appends a single Interface to the end of the `interfaces` array for an existing Configuration Profile.
    ///
    /// - The User accessing this operation must have `read_write` grants to the Linode.
    /// - A successful request triggers a `linode_config_update` event.
    /// - If the new Interface is added with `"primary": true`, then any existing primary Interface is changed to `"primary": false`.
    ///
    /// Reboot the Linode with this Configuration Profile to activate an Interface that was added with this operation.
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
    ///     linode-cli linodes config-interface-add $linodeId $configId \
    ///   --purpose vpc \
    ///   --primary false \
    ///   --subnet_id 101 \
    ///   --ipv4.vpc "10.0.1.2" \
    ///   --ipv4.nat_1_1 "203.0.113.2"
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
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/configs/{}/interfaces", & request
                    .api_version, & request.linode_id, & request.config_id
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
            crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesResponse,
        >(response)
            .await
    }
    /// Updates a `vpc` or `public` purpose Interface for this Configuration Profile.
    ///
    /// - The User accessing this operation must have `read_write` grants to the Linode.
    /// - A successful request triggers a `linode_config_update` event.
    /// - The Interface `purpose` cannot be updated with this operation.
    /// - VPC Subnets cannot be updated on an Interface. A new `vpc` purpose Interface must be created to assign a different Subnet to a Configuration Profile.
    /// - Only `primary` can be updated for `public` purpose Interfaces.
    /// - This operation not currently allowed for `vlan` purpose Interfaces.
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
    ///     linode-cli linodes config-interface-update $linodeId $configId $interfaceId \
    ///   --primary true \
    ///   --ipv4.vpc "10.0.1.2" \
    ///   --ipv4.nat_1_1 "203.0.113.2"
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
    ) -> crate::SdkResult<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/linode/instances/{}/configs/{}/interfaces/{}", & request
                    .api_version, & request.linode_id, & request.config_id, & request
                    .interface_id
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
            crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponse,
        >(response)
            .await
    }
}
