#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .images()
        .delete(linode_api::resources::images::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionImagesImageIdApiVersionEnum::V4,
            image_id: "linode/debian11".to_string(),
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
        .images()
        .list(linode_api::resources::images::ListRequest {
            api_version: linode_api::models::GetApiVersionImagesApiVersionEnum::V4,
            ..Default::default()
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
        .images()
        .get(linode_api::resources::images::GetRequest {
            api_version: linode_api::models::GetApiVersionImagesImageIdApiVersionEnum::V4,
            image_id: "linode/debian11".to_string(),
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
        .images()
        .create(linode_api::resources::images::CreateRequest {
            api_version: linode_api::models::PostApiVersionImagesApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionImagesBody {
                cloud_init: Some(true),
                description: Some("string".to_string()),
                disk_id: 42,
                label: Some("string".to_string()),
                tags: Some(vec!["repair-image".to_string(), "fix-1".to_string()]),
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
        .images()
        .put(linode_api::resources::images::PutRequest {
            api_version: linode_api::models::PutApiVersionImagesImageIdApiVersionEnum::V4,
            image_id: "linode/debian11".to_string(),
            data: linode_api::models::PutApiVersionImagesImageIdBody {
                capabilities: Some(
                    vec!["cloud-init".to_string(), "distributed-sites".to_string()],
                ),
                created: Some("2021-08-14T22:44:02".to_string()),
                created_by: Some("linode".to_string()),
                deprecated: Some(false),
                description: linode_api::Patch::new(
                    "Example image description.".to_string(),
                ),
                eol: linode_api::Patch::new("2026-07-01T04:00:00".to_string()),
                expiry: linode_api::Patch::new("1970-01-01T00:00:00".to_string()),
                id: Some("linode/debian11".to_string()),
                is_public: Some(true),
                label: Some("Debian 11".to_string()),
                regions: Some(
                    vec![
                        linode_api::models::PutApiVersionImagesImageIdBodyRegionsItem {
                        region : Some("us-iad".to_string()), status :
                        Some(linode_api::models::PutApiVersionImagesImageIdBodyRegionsItemStatusEnum::Available),
                        ..Default::default() }
                    ],
                ),
                size: Some(2500),
                status: Some(
                    linode_api::models::PutApiVersionImagesImageIdBodyStatusEnum::Available,
                ),
                tags: Some(vec!["repair-image".to_string(), "fix-1".to_string()]),
                total_size: Some(1234567),
                type_field: Some(
                    linode_api::models::PutApiVersionImagesImageIdBodyTypeEnum::Manual,
                ),
                updated: Some("2021-08-14T22:44:02".to_string()),
                vendor: linode_api::Patch::new("Debian".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
