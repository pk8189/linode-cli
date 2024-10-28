#[derive(Clone, Debug)]
pub struct KeysClient {
    base_client: crate::core::base_client::BaseClient,
}
impl KeysClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Revokes an Object Storage Key. This keypair will no longer be usable by third-party clients.
    ///
    /// - A successful request triggers an `obj_access_key_delete` event.
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
    ///     linode-cli object-storage keys-delete 12345
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
                    "/{}/object-storage/keys/{}", & request.api_version, & request.key_id
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
    /// Returns a paginated list of Object Storage keys for authenticating to the Object Storage S3 API.
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
    ///     linode-cli object-storage keys-list
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
    ) -> crate::SdkResult<crate::models::GetApiVersionObjectStorageKeysResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/object-storage/keys", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionObjectStorageKeysResponse,
        >(response)
            .await
    }
    /// Returns a single Object Storage key provisioned for your account.
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
    ///     linode-cli object-storage keys-view \
    ///   --keyId 12345
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     object_storage:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn get(
        &self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionObjectStorageKeysKeyIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/keys/{}", & request.api_version, & request.key_id
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
            crate::models::GetApiVersionObjectStorageKeysKeyIdResponse,
        >(response)
            .await
    }
    /// Provisions a new Object Storage key for authenticating to the Object Storage S3 API. A successful request triggers an `obj_access_key_create` [event](https://techdocs.akamai.com/linode-api/reference/get-events).
    ///
    /// > 📘
    /// >
    /// > Accounts with negative balances can't access this operation.
    ///
    /// **The `regions` and `region` parameters**
    ///
    /// When creating an Object Storage key, specify one or more data centers ([regions](https://techdocs.akamai.com/linode-api/reference/get-regions)) where you want to create and manage Object Storage buckets.
    ///
    /// - **The `regions` array**. Populate it with `regionId` values. The resulting Object Storage key grants access to list and create new buckets in these regions. This *doesn't* give access to manage content in these buckets. To address this, you can:
    ///
    ///   - Use the `bucket_access` array instead to grant management access, per bucket.
    ///
    ///   - Use [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/) to change the access for this key.
    ///
    /// - **The `bucket_access` array**. This optional array lets you set up limited keys. Include individual objects naming a `regionId`, the target `bucket_name`, and the `permissions` for the Object Storage key. Use the resulting key to manage content in the `bucket_name`, based on the permission level set. You can also use the key to create new buckets in the named region. However, the key doesn't have access to manage content in the newly created bucket. You can grant it this access using [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/).
    ///
    /// - **Combine the two to apply varying levels of access in the key**. For example, set `regions` to `us-west` to give the key bucket list and create access in that region. Then, set up the `bucket_access` array to give access to a specific `bucket_name` in the `us-east` region. The key has access to manage content in that `bucket_name` and list and create buckets in the `us-east` region, too. If you include the same region in both, the settings applied in the `bucket_access` array take precedence. For example, assume you include `us-east` in the `regions` array, expecting to only give bucket list and creation access to that region. If you also set `us-east` as a `region` in the `bucket_access` array, the Object Storage key gives access to manage content in the specified `bucket_name`, and lets you list and create buckets in that region.
    ///
    /// **The `cluster` parameter (legacy)**
    ///
    /// For backward compatibility, include the `cluster` parameter to create an Object Storage key. Use the `clusterId` equivalent (us-west-1) instead of the `regionId` (us-west). Leave the `regions` array out. If including the `bucket_access` array to limit access, omit `region` from each object. Use the resulting key in clusters in all supported regions.
    ///
    /// > 📘
    /// >
    /// > While the API supports this method, you should use the `regions` parameters, instead.
    ///
    /// - **Unlimited access**. Omit the `bucket_access` array. The Object Storage key has unlimited cluster access to all buckets, with all permissions.
    ///
    /// - **Limited access**. Include the `bucket_access` array. Set the target `bucket_name` and the level of `permissions` for access to that bucket. Use the resulting key to manage content in the named bucket. A limited Object Storage key can [list all buckets](https://techdocs.akamai.com/linode-api/reference/get-object-storage-buckets) and [create a new bucket](https://techdocs.akamai.com/linode-api/reference/post-object-storage-bucket). However, you can't use the key to perform any actions on a bucket, unless the key has access to it. You can use [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/) to modify a key's access.
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
    ///     linode-cli object-storage keys-create \
    ///   --label "my-object-storage-key" \
    ///   --bucket_access '[{"region": "ap-south", "bucket_name": "bucket-example-1", "permissions": "read_write" }]'
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
    ) -> crate::SdkResult<crate::models::PostApiVersionObjectStorageKeysResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/object-storage/keys", & request.api_version));
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
            crate::models::PostApiVersionObjectStorageKeysResponse,
        >(response)
            .await
    }
    /// Updates an Object Storage key on your account. A successful request triggers an `obj_access_key_update` [event](https://techdocs.akamai.com/linode-api/reference/get-events).
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
    ///     linode-cli object-storage keys-update \
    ///   --keyId 12345
    ///   --label "my-object-storage-key"
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
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
    ) -> crate::SdkResult<crate::models::PutApiVersionObjectStorageKeysKeyIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/object-storage/keys/{}", & request.api_version, & request.key_id
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
            crate::models::PutApiVersionObjectStorageKeysKeyIdResponse,
        >(response)
            .await
    }
}
