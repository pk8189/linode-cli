#[derive(Clone, Debug)]
pub struct BucketsClient {
    base_client: crate::core::base_client::BaseClient,
}
impl BucketsClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn ssl_resource(
        &self,
    ) -> crate::resources::object_storage::buckets::ssl_resource::resource_client::SslClient {
        crate::resources::object_storage::buckets::ssl_resource::resource_client::SslClient::new(
            self.base_client.clone(),
        )
    }
    pub fn object_acl(
        &self,
    ) -> crate::resources::object_storage::buckets::object_acl::resource_client::ObjectAclClient {
        crate::resources::object_storage::buckets::object_acl::resource_client::ObjectAclClient::new(
            self.base_client.clone(),
        )
    }
    pub fn object_list(
        &self,
    ) -> crate::resources::object_storage::buckets::object_list::resource_client::ObjectListClient {
        crate::resources::object_storage::buckets::object_list::resource_client::ObjectListClient::new(
            self.base_client.clone(),
        )
    }
    pub fn access(
        &self,
    ) -> crate::resources::object_storage::buckets::access::resource_client::AccessClient {
        crate::resources::object_storage::buckets::access::resource_client::AccessClient::new(
            self.base_client.clone(),
        )
    }
    pub fn object_url(
        &self,
    ) -> crate::resources::object_storage::buckets::object_url::resource_client::ObjectUrlClient {
        crate::resources::object_storage::buckets::object_url::resource_client::ObjectUrlClient::new(
            self.base_client.clone(),
        )
    }
    /// Removes a single bucket.
    ///
    /// > 📘
    /// >
    /// > - You need to remove all objects from a bucket before you can delete it. While you *can* delete a bucket using the [s3cmd command-line tool](https://www.linode.com/docs/products/storage/object-storage/guides/s3cmd/#delete-a-bucket), this operation fails if the bucket contains too many objects. The best way to empty large buckets is to use the [S3 API to configure lifecycle policies](https://docs.ceph.com/en/latest/radosgw/bucketpolicy/#). Set a policy to remove all objects, wait a day or more for the system to remove all objects, then delete the bucket.
    /// >
    /// > - The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#delete-bucket) equivalent operation offers more detail.
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
    pub async fn delete(
        &self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<serde_json::Value> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/buckets/{}/{}", & request.api_version, & request
                    .region_id, & request.bucket
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
    /// Returns a paginated list of all Object Storage buckets available in your account.
    ///
    /// > 📘
    /// >
    /// > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/serviceops/#list-buckets) equivalent operation offers more detail.
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
    ) -> crate::SdkResult<crate::models::GetApiVersionObjectStorageBucketsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/object-storage/buckets", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionObjectStorageBucketsResponse,
        >(response)
            .await
    }
    /// Returns a list of buckets on your account, in the specified region.
    ///
    /// > 📘
    /// >
    /// > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#get-bucket) equivalent operation offers more detail.
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
    pub async fn get_by_region_id(
        &self,
        request: super::request_types::GetByRegionIdRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionObjectStorageBucketsRegionIdResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/buckets/{}", & request.api_version, & request
                    .region_id
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
            crate::models::GetApiVersionObjectStorageBucketsRegionIdResponse,
        >(response)
            .await
    }
    /// Returns a single Object Storage bucket.
    ///
    /// > 📘
    /// >
    /// > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#get-bucket) equivalent operation offers more detail.
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
    pub async fn get_by_bucket(
        &self,
        request: super::request_types::GetByBucketRequest,
    ) -> crate::SdkResult<
        crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/buckets/{}/{}", & request.api_version, & request
                    .region_id, & request.bucket
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
            crate::models::GetApiVersionObjectStorageBucketsRegionIdBucketResponse,
        >(response)
            .await
    }
    /// Creates an Object Storage bucket in the specified data center ([region](https://techdocs.akamai.com/linode-api/reference/get-regions)). If the bucket already exists on your account, this operation returns a 200 response with that bucket as if the API just created it.
    ///
    /// > 📘
    /// >
    /// > - Accounts with negative balances can't access this operation.
    /// >
    /// > - The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket) equivalent operation offers more detail.
    /// >
    /// > - The API still supports the `clusterId` equivalent (`us-west-1`) when setting a `region` for a new bucket, but you should use the `regionId` (`us-west`), instead.
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
    ) -> crate::SdkResult<crate::models::PostApiVersionObjectStorageBucketsResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/object-storage/buckets", & request.api_version));
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
            crate::models::PostApiVersionObjectStorageBucketsResponse,
        >(response)
            .await
    }
}
