#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .buckets()
        .delete(linode_api::resources::object_storage::buckets::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionObjectStorageBucketsRegionIdBucketApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
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
        .list(linode_api::resources::object_storage::buckets::ListRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageBucketsApiVersionEnum::V4,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_by_region_id_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .buckets()
        .get_by_region_id(linode_api::resources::object_storage::buckets::GetByRegionIdRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdApiVersionEnum::V4,
            region_id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_by_bucket_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .buckets()
        .get_by_bucket(linode_api::resources::object_storage::buckets::GetByBucketRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
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
        .create(linode_api::resources::object_storage::buckets::CreateRequest {
            api_version: linode_api::models::PostApiVersionObjectStorageBucketsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionObjectStorageBucketsBody {
                acl: Some(
                    linode_api::models::PostApiVersionObjectStorageBucketsBodyAclEnum::Private,
                ),
                cors_enabled: Some(false),
                label: "example-bucket".to_string(),
                region: Some("us-east".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
