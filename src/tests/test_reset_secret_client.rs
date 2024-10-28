#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .oauth_clients()
        .reset_secret()
        .create(linode_api::resources::account::oauth_clients::reset_secret::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountOauthClientsClientIdResetSecretApiVersionEnum::V4,
            client_id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
