#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .managed()
        .services()
        .delete(linode_api::resources::managed::services::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionManagedServicesServiceIdApiVersionEnum::V4,
            service_id: 123,
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
        .managed()
        .services()
        .list(linode_api::resources::managed::services::ListRequest {
            api_version: linode_api::models::GetApiVersionManagedServicesApiVersionEnum::V4,
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
        .managed()
        .services()
        .get(linode_api::resources::managed::services::GetRequest {
            api_version: linode_api::models::GetApiVersionManagedServicesServiceIdApiVersionEnum::V4,
            service_id: 123,
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
        .managed()
        .services()
        .create(linode_api::resources::managed::services::CreateRequest {
            api_version: linode_api::models::PostApiVersionManagedServicesApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionManagedServicesBody {
                address: Some("https://example.org".to_string()),
                body: linode_api::Patch::new("it worked".to_string()),
                consultation_group: Some("on-call".to_string()),
                created: Some("2018-01-01T00:01:01".to_string()),
                credentials: Some(vec![9991]),
                id: Some(9944),
                label: Some("prod-1".to_string()),
                notes: linode_api::Patch::new(
                    "The service name is my-cool-application".to_string(),
                ),
                region: Some("string".to_string()),
                service_type: Some(
                    linode_api::models::PostApiVersionManagedServicesBodyServiceTypeEnum::Url,
                ),
                status: Some(
                    linode_api::models::PostApiVersionManagedServicesBodyStatusEnum::Ok,
                ),
                timeout: Some(30),
                updated: Some("2018-03-01T00:01:01".to_string()),
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
        .managed()
        .services()
        .put(linode_api::resources::managed::services::PutRequest {
            api_version: linode_api::models::PutApiVersionManagedServicesServiceIdApiVersionEnum::V4,
            service_id: 123,
            data: linode_api::models::PutApiVersionManagedServicesServiceIdBody {
                address: Some("https://example.org".to_string()),
                body: linode_api::Patch::new("it worked".to_string()),
                consultation_group: Some("on-call".to_string()),
                created: Some("2018-01-01T00:01:01".to_string()),
                credentials: Some(vec![9991]),
                id: Some(9944),
                label: Some("prod-1".to_string()),
                notes: linode_api::Patch::new(
                    "The service name is my-cool-application".to_string(),
                ),
                region: Some("string".to_string()),
                service_type: Some(
                    linode_api::models::PutApiVersionManagedServicesServiceIdBodyServiceTypeEnum::Url,
                ),
                status: Some(
                    linode_api::models::PutApiVersionManagedServicesServiceIdBodyStatusEnum::Ok,
                ),
                timeout: Some(30),
                updated: Some("2018-03-01T00:01:01".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
