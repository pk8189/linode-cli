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
        .access()
        .create(linode_api::resources::object_storage::buckets::access::CreateRequest {
            api_version: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
            data: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessBody {
                acl: Some(
                    linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::Private,
                ),
                cors_enabled: Some(true),
                ..Default::default()
            },
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
        .access()
        .put(linode_api::resources::object_storage::buckets::access::PutRequest {
            api_version: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
            data: linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessBody {
                acl: Some(
                    linode_api::models::PutApiVersionObjectStorageBucketsRegionIdBucketAccessBodyAclEnum::Private,
                ),
                cors_enabled: Some(true),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
