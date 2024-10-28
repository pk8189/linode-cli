#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .settings()
        .list(linode_api::resources::account::settings::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountSettingsApiVersionEnum::V4,
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
        .account()
        .settings()
        .put(linode_api::resources::account::settings::PutRequest {
            api_version: linode_api::models::PutApiVersionAccountSettingsApiVersionEnum::V4,
            data: linode_api::models::PutApiVersionAccountSettingsBody {
                backups_enabled: Some(true),
                longview_subscription: Some("longview-3".to_string()),
                managed: Some(true),
                network_helper: Some(false),
                object_storage: Some(
                    linode_api::models::PutApiVersionAccountSettingsBodyObjectStorageEnum::Active,
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
