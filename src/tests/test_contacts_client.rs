#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .managed()
        .contacts()
        .delete(linode_api::resources::managed::contacts::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionManagedContactsContactIdApiVersionEnum::V4,
            contact_id: 123,
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
        .contacts()
        .list(linode_api::resources::managed::contacts::ListRequest {
            api_version: linode_api::models::GetApiVersionManagedContactsApiVersionEnum::V4,
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
        .managed()
        .contacts()
        .get(linode_api::resources::managed::contacts::GetRequest {
            api_version: linode_api::models::GetApiVersionManagedContactsContactIdApiVersionEnum::V4,
            contact_id: 123,
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
        .contacts()
        .create(linode_api::resources::managed::contacts::CreateRequest {
            api_version: linode_api::models::PostApiVersionManagedContactsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionManagedContactsBody {
                email: Some("john.doe@example.org".to_string()),
                group: linode_api::Patch::new("on-call".to_string()),
                id: Some(567),
                name: Some("John Doe".to_string()),
                phone: Some(linode_api::models::PostApiVersionManagedContactsBodyPhone {
                    primary: linode_api::Patch::new("123-456-7890".to_string()),
                    secondary: linode_api::Patch::new("string".to_string()),
                    ..Default::default()
                }),
                updated: Some("2018-01-01T00:01:01".to_string()),
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
        .contacts()
        .put(linode_api::resources::managed::contacts::PutRequest {
            api_version: linode_api::models::PutApiVersionManagedContactsContactIdApiVersionEnum::V4,
            contact_id: 123,
            data: linode_api::models::PutApiVersionManagedContactsContactIdBody {
                email: Some("john.doe@example.org".to_string()),
                group: linode_api::Patch::new("on-call".to_string()),
                id: Some(567),
                name: Some("John Doe".to_string()),
                phone: Some(linode_api::models::PutApiVersionManagedContactsContactIdBodyPhone {
                    primary: linode_api::Patch::new("123-456-7890".to_string()),
                    secondary: linode_api::Patch::new("string".to_string()),
                    ..Default::default()
                }),
                updated: Some("2018-01-01T00:01:01".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
