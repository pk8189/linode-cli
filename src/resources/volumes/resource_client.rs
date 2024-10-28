#[derive(Clone, Debug)]
pub struct VolumesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl VolumesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn types_resource(
        &self,
    ) -> crate::resources::volumes::types_resource::resource_client::TypesClient {
        crate::resources::volumes::types_resource::resource_client::TypesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn attach(
        &self,
    ) -> crate::resources::volumes::attach::resource_client::AttachClient {
        crate::resources::volumes::attach::resource_client::AttachClient::new(
            self.base_client.clone(),
        )
    }
    pub fn clone(
        &self,
    ) -> crate::resources::volumes::clone::resource_client::CloneClient {
        crate::resources::volumes::clone::resource_client::CloneClient::new(
            self.base_client.clone(),
        )
    }
    pub fn detach(
        &self,
    ) -> crate::resources::volumes::detach::resource_client::DetachClient {
        crate::resources::volumes::detach::resource_client::DetachClient::new(
            self.base_client.clone(),
        )
    }
    pub fn resize(
        &self,
    ) -> crate::resources::volumes::resize::resource_client::ResizeClient {
        crate::resources::volumes::resize::resource_client::ResizeClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a Volume you have permission to `read_write`.
    ///
    /// - __Deleting a Volume is a destructive action and cannot be undone.__
    ///
    /// - Deleting stops billing for the Volume. You will be billed for time used within the billing period the Volume was active.
    ///
    /// - Volumes that are migrating cannot be deleted until the migration is finished.
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
    ///     linode-cli volumes delete 12345
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     volumes:read_write
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
                &format!("/{}/volumes/{}", & request.api_version, & request.volume_id),
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
    /// Returns a paginated list of Volumes you have permission to view.
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
    ///     linode-cli volumes list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     volumes:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionVolumesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/volumes", & request.api_version));
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
            crate::models::GetApiVersionVolumesResponse,
        >(response)
            .await
    }
    /// Get information about a single Volume.
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
    ///     linode-cli volumes view 12345
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     volumes:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionVolumesVolumeIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/volumes/{}", & request.api_version, & request.volume_id),
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
        crate::core::response::process_json::<
            crate::models::GetApiVersionVolumesVolumeIdResponse,
        >(response)
            .await
    }
    /// Creates a volume on your account. For this to complete, you need the `add_volumes` grant. Creating a new volume accrues additional charges on your account.
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
    ///     linode-cli volumes create \
    ///   --label my-volume \
    ///   --size 20 \
    ///   --linode_id 12346 \
    ///   --encryption enabled \
    ///   --no-defaults
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     volumes:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostApiVersionVolumesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/volumes", & request.api_version));
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
            crate::models::PostApiVersionVolumesResponse,
        >(response)
            .await
    }
    /// Updates a Volume that you have permission to `read_write`.
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
    ///     linode-cli volumes update 12345 \
    ///   --label my_volume
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     volumes:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<crate::models::PutApiVersionVolumesVolumeIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!("/{}/volumes/{}", & request.api_version, & request.volume_id),
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
            crate::models::PutApiVersionVolumesVolumeIdResponse,
        >(response)
            .await
    }
}
