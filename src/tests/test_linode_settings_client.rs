#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .managed()
        .linode_settings()
        .list(linode_api::resources::managed::linode_settings::ListRequest {
            api_version: linode_api::models::GetApiVersionManagedLinodeSettingsApiVersionEnum::V4,
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
        .linode_settings()
        .get(linode_api::resources::managed::linode_settings::GetRequest {
            api_version: linode_api::models::GetApiVersionManagedLinodeSettingsLinodeIdApiVersionEnum::V4,
            linode_id: 123,
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
        .linode_settings()
        .put(linode_api::resources::managed::linode_settings::PutRequest {
            api_version: linode_api::models::PutApiVersionManagedLinodeSettingsLinodeIdApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PutApiVersionManagedLinodeSettingsLinodeIdBody {
                group: Some("linodes".to_string()),
                id: Some(123),
                label: Some("linode123".to_string()),
                ssh: Some(linode_api::models::PutApiVersionManagedLinodeSettingsLinodeIdBodySsh {
                    access: Some(true),
                    ip: Some("203.0.113.1".to_string()),
                    port: linode_api::Patch::new(22),
                    user: linode_api::Patch::new("linode".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
