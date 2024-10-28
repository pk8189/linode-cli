#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .service_transfers()
        .accept()
        .create(linode_api::resources::account::service_transfers::accept::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountServiceTransfersTokenAcceptApiVersionEnum::V4,
            token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
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
        .account()
        .entity_transfers()
        .accept()
        .create(linode_api::resources::account::entity_transfers::accept::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountEntityTransfersTokenAcceptApiVersionEnum::V4,
            token: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
