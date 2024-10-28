#[derive(Clone, Debug)]
pub struct ObjectUrlClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ObjectUrlClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Creates a pre-signed URL to access a single object in a bucket. Use it to share, create, or delete objects by using the appropriate HTTP method in your request body's `method` parameter.
    ///
    /// > 📘
    /// >
    /// > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/) equivalent operation offers more detail.
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
    ///     object_storage:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionObjectStorageBucketsRegionIdBucketObjectUrlResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/buckets/{}/{}/object-url", & request.api_version,
                    & request.region_id, & request.bucket
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
            crate::models::PostApiVersionObjectStorageBucketsRegionIdBucketObjectUrlResponse,
        >(response)
            .await
    }
}
