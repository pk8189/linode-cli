#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .service_transfers()
        .delete(linode_api::resources::account::service_transfers::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionAccountServiceTransfersTokenApiVersionEnum::V4,
            token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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
        .account()
        .service_transfers()
        .list(linode_api::resources::account::service_transfers::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountServiceTransfersApiVersionEnum::V4,
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
        .account()
        .service_transfers()
        .get(linode_api::resources::account::service_transfers::GetRequest {
            api_version: linode_api::models::GetApiVersionAccountServiceTransfersTokenApiVersionEnum::V4,
            token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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
        .service_transfers()
        .create(linode_api::resources::account::service_transfers::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountServiceTransfersApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountServiceTransfersBody {
                entities: linode_api::models::PostApiVersionAccountServiceTransfersBodyEntities {
                    linodes: Some(vec![111, 222]),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
