#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .child_accounts()
        .token_resource()
        .create(linode_api::resources::account::child_accounts::token_resource::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountChildAccountsEuuidTokenApiVersionEnum::V4,
            euuid: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
