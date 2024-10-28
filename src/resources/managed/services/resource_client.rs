#[derive(Clone, Debug)]
pub struct ServicesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ServicesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn disable(
        &self,
    ) -> crate::resources::managed::services::disable::resource_client::DisableClient {
        crate::resources::managed::services::disable::resource_client::DisableClient::new(
            self.base_client.clone(),
        )
    }
    pub fn enable(
        &self,
    ) -> crate::resources::managed::services::enable::resource_client::EnableClient {
        crate::resources::managed::services::enable::resource_client::EnableClient::new(
            self.base_client.clone(),
        )
    }
    /// Deletes a Managed Service.  This service will no longer be monitored by Linode Managed.
    ///
    /// This operation can only be accessed by the unrestricted users of an account.
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
    ///     linode-cli managed service-delete 9994
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_write
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
                    "/{}/managed/services/{}", & request.api_version, & request
                    .service_id
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
    /// Returns a paginated list of Managed Services on your Account. These are the services Linode Managed is monitoring and will report and attempt to resolve issues with.
    ///
    /// This operation can only be accessed by the unrestricted users of an account.
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
    ///     linode-cli managed services-list
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
    ) -> crate::SdkResult<crate::models::GetApiVersionManagedServicesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/managed/services", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionManagedServicesResponse,
        >(response)
            .await
    }
    /// Returns information about a single Managed Service on your Account.
    ///
    /// This operation can only be accessed by the unrestricted users of an account.
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
    ///     linode-cli managed service-view 9994
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
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionManagedServicesServiceIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/managed/services/{}", & request.api_version, & request
                    .service_id
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
            crate::models::GetApiVersionManagedServicesServiceIdResponse,
        >(response)
            .await
    }
    /// Creates a Managed Service. Linode Managed will begin monitoring this service and reporting and attempting to resolve any Issues.
    ///
    /// This operation can only be accessed by the unrestricted users of an account.
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
    ///     linode-cli managed service-create \
    ///   --service_type url \
    ///   --label prod-1 \
    ///   --address "https://example.org" \
    ///   --timeout 30 \
    ///   --body "it worked" \
    ///   --consultation_group on-call \
    ///   --notes "The service name is \
    ///     my-cool-application" \
    ///   --credentials 9991
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
    ) -> crate::SdkResult<crate::models::PostApiVersionManagedServicesResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/managed/services", & request.api_version));
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
            crate::models::PostApiVersionManagedServicesResponse,
        >(response)
            .await
    }
    /// Updates information about a Managed Service.
    ///
    /// This operation can only be accessed by the unrestricted users of an account.
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
    ///     linode-cli managed service-update 9994 \
    ///   --service_type url \
    ///   --label prod-1 \
    ///   --address "https://example.org" \
    ///   --timeout 30 \
    ///   --body "it worked" \
    ///   --consultation_group on-call \
    ///   --notes "The service name is my-cool-application" \
    ///   --credentials 9991
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
    ) -> crate::SdkResult<crate::models::PutApiVersionManagedServicesServiceIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/managed/services/{}", & request.api_version, & request
                    .service_id
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
            crate::models::PutApiVersionManagedServicesServiceIdResponse,
        >(response)
            .await
    }
}
