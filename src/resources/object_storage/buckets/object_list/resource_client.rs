#[derive(Clone, Debug)]
pub struct ObjectListClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ObjectListClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Returns the contents of a bucket. The contents are paginated using a `marker`, that's the name of the last object on the previous page.  Objects can also be filtered by `prefix` and `delimiter`. See [Filtering and sorting](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) for more information.
    ///
    /// > 📘
    /// >
    /// > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#get-object) equivalent operation offers more detail.
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
    ///     object_storage:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectListResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/buckets/{}/{}/object-list", & request
                    .api_version, & request.region_id, & request.bucket
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add_option("delimiter", &request.delimiter, false);
        queries.add_option("marker", &request.marker, false);
        queries.add_option("page_size", &request.page_size, false);
        queries.add_option("prefix", &request.prefix, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectListResponse,
        >(response)
            .await
    }
}
