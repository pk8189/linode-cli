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
        .object_acl()
        .list(linode_api::resources::object_storage::buckets::object_acl::ListRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketObjectAclApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
            name: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_put_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .buckets()
        .object_acl()
        .put(linode_api::resources::object_storage::buckets::object_acl::PutRequest {
            api_version: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
            data: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBody {
                acl: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketObjectAclBodyAclEnum::PublicRead,
                name: "string".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
