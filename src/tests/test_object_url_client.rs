#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .buckets()
        .object_url()
        .create(linode_api::resources::object_storage::buckets::object_url::CreateRequest {
            api_version: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketObjectUrlApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
            data: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketObjectUrlBody {
                content_type: Some("string".to_string()),
                expires_in: Some(123),
                method: "GET".to_string(),
                name: "example".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
