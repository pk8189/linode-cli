#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .cancel()
        .create(linode_api::resources::object_storage::cancel::CreateRequest {
            api_version: linode_api::models::PostApiVersionObjectStorageCancelApiVersionEnum::V4,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .backups()
        .cancel()
        .create(linode_api::resources::linode::instances::backups::cancel::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsCancelApiVersionEnum::V4,
            linode_id: 123,
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
        .account()
        .cancel()
        .create(linode_api::resources::account::cancel::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountCancelApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountCancelBody {
                comments: Some(
                    "I'm consolidating multiple accounts into one.".to_string(),
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
