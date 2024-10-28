#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .agreements()
        .list(linode_api::resources::account::agreements::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountAgreementsApiVersionEnum::V4,
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
        .agreements()
        .create(linode_api::resources::account::agreements::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountAgreementsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountAgreementsBody {
                eu_model: Some(true),
                master_service_agreement: Some(true),
                privacy_policy: Some(true),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
