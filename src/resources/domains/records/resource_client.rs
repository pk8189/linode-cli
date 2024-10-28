#[derive(Clone, Debug)]
pub struct RecordsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RecordsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Deletes a Record on this Domain.
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
    ///     linode-cli domains records-delete 123 234
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     domains:read_write
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
                    "/{}/domains/{}/records/{}", & request.api_version, & request
                    .domain_id, & request.record_id
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
    /// Returns a paginated list of Records configured on a Domain in Linode's DNS Manager.
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
    ///     linode-cli domains records-list 1234
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     domains:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionDomainsDomainIdRecordsResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/domains/{}/records", & request.api_version, & request.domain_id
                ),
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
            crate::models::GetApiVersionDomainsDomainIdRecordsResponse,
        >(response)
            .await
    }
    /// View a single Record on this Domain.
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
    ///     linode-cli domains records-view 123 234
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     domains:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionDomainsDomainIdRecordsRecordIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/domains/{}/records/{}", & request.api_version, & request
                    .domain_id, & request.record_id
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
            crate::models::GetApiVersionDomainsDomainIdRecordsRecordIdResponse,
        >(response)
            .await
    }
    /// Adds a new Domain Record to the zonefile this Domain represents.
    ///
    /// Each domain can have up to 12,000 active records.
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
    ///     linode-cli domains records-create 123 \
    ///   --type A \
    ///   --name test \
    ///   --target 203.0.113.1 \
    ///   --priority 50 \
    ///   --weight 50 \
    ///   --port 80 \
    ///   --ttl_sec 604800
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     domains:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostApiVersionDomainsDomainIdRecordsResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/domains/{}/records", & request.api_version, & request.domain_id
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
            crate::models::PostApiVersionDomainsDomainIdRecordsResponse,
        >(response)
            .await
    }
    /// Updates a single Record on this Domain.
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
    ///     linode-cli domains records-update 123 234 \
    ///   --name test \
    ///   --target 203.0.113.1 \
    ///   --priority 50 \
    ///   --weight 50 \
    ///   --port 80 \
    ///   --ttl_sec 604800
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     domains:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionDomainsDomainIdRecordsRecordIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/domains/{}/records/{}", & request.api_version, & request
                    .domain_id, & request.record_id
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
            crate::models::PutApiVersionDomainsDomainIdRecordsRecordIdResponse,
        >(response)
            .await
    }
}
