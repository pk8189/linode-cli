#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .keys()
        .delete(linode_api::resources::object_storage::keys::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionObjectStorageKeysKeyIdApiVersionEnum::V4,
            key_id: 123,
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
        .keys()
        .list(linode_api::resources::object_storage::keys::ListRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageKeysApiVersionEnum::V4,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .keys()
        .get(linode_api::resources::object_storage::keys::GetRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageKeysKeyIdApiVersionEnum::V4,
            key_id: 123,
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
        .keys()
        .create(linode_api::resources::object_storage::keys::CreateRequest {
            api_version: linode_api::models::PostApiVersionObjectStorageKeysApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionObjectStorageKeysBody {
                bucket_access: Some(
                    vec![
                        linode_api::models::PostApiVersionObjectStorageKeysBodyBucketAccessItem
                        { bucket_name : Some("example-bucket".to_string()), permissions :
                        Some(linode_api::models::PostApiVersionObjectStorageKeysBodyBucketAccessItemPermissionsEnum::ReadWrite),
                        region : Some("us-iad".to_string()), ..Default::default() }
                    ],
                ),
                label: Some("my-key".to_string()),
                regions: Some(vec!["us-iad".to_string()]),
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
        .keys()
        .put(linode_api::resources::object_storage::keys::PutRequest {
            api_version: linode_api::models::PutApiVersionObjectStorageKeysKeyIdApiVersionEnum::V4,
            key_id: 123,
            data: linode_api::models::PutApiVersionObjectStorageKeysKeyIdBody {
                label: Some("my-key".to_string()),
                regions: Some(vec!["us-iad".to_string(), "fr-par".to_string()]),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
