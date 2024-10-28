#[derive(Clone, Debug)]
pub struct ObjectAclClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ObjectAclClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// View an Object's configured Access Control List (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects and specify the level of access granted to those users.
    ///
    /// > 📘
    /// >
    /// > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#get-object-acl) equivalent operation offers more detail.
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
        crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectAclResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/buckets/{}/{}/object-acl", & request.api_version,
                    & request.region_id, & request.bucket
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("name", &request.name, false);
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectAclResponse,
        >(response)
            .await
    }
    /// Update an object's configured access control level (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects and specify the level of access granted to those users.
    ///
    /// > 📘
    /// >
    /// > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#set-object-acl) equivalent operation offers more detail.
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
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<
        crate::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/buckets/{}/{}/object-acl", & request.api_version,
                    & request.region_id, & request.bucket
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
            crate::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclResponse,
        >(response)
            .await
    }
}
