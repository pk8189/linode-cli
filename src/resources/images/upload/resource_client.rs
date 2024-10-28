#[derive(Clone, Debug)]
pub struct UploadClient {
    base_client: crate::core::base_client::BaseClient,
}
impl UploadClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Creates a new private image container and returns a URL as the `upload_to` object in the response. Use this URL to upload your own disk image to the container.
    ///
    /// 1. Ensure the disk image is raw disk image (`.img`) format.
    ///
    /// 2. Compress the disk image using gzip (`.gz`) format. Compressed, the file can be up to 5 GB and decompressed it can be up to 6 GB.
    ///
    /// 3. Upload the file in a separate PUT request that includes the `Content-type: application/octet-stream` header:
    ///
    ///   `-`-`
    ///   curl -v \
    ///     -H "Content-Type: application/octet-stream" \
    ///     --upload-file example.img.gz \
    ///     $UPLOAD_URL \
    ///     --progress-bar \
    ///     --output /dev/null
    ///   `-`-`
    ///
    /// > 📘
    /// >
    /// > - You need to upload image data within 24 hours of creation or the API cancels the upload and deletes the image container.
    /// >
    /// > - Only core regions that support our [Object Storage](https://techdocs.akamai.com/cloud-computing/reference/how-to-choose-a-data-center#product-availability) service can store an uploaded image.
    /// >
    /// > - If you create an image from an encrypted disk, the API doesn't encrypt the image. When you rebuild a compute instance from that image, the resulting disk will be automatically encrypted.
    /// >
    /// > - You can create a new image and upload image data using a single process through [Cloud Manager](https://www.linode.com/docs/products/tools/images/guides/upload-an-image/#uploading-an-image-file-through-the-cloud-manager) or the [Linode CLI](https://www.linode.com/docs/products/tools/images/guides/upload-an-image/#uploading-an-image-file-through-the-linode-cli).
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
    ///     # Run the operation to just get the upload_to URL
    /// linode-cli images upload \
    ///   --description "Optional details about the Image" \
    ///   --label "Example Image" \
    ///   --region us-east
    ///
    /// # Upload the image file in a single step
    /// linode-cli image-upload \
    ///   --description "Optional details about the Image" \
    ///   --label "Example Image" \
    ///   --region us-east \
    ///   /path/to/image-file.img.gz
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     images:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostApiVersionImagesUploadResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/images/upload", & request.api_version));
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
            crate::models::PostApiVersionImagesUploadResponse,
        >(response)
            .await
    }
}
