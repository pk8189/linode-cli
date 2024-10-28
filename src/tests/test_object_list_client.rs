#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .buckets()
        .object_list()
        .list(linode_api::resources::object_storage::buckets::object_list::ListRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectListApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
