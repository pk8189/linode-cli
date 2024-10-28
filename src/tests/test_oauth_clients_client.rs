#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .oauth_clients()
        .delete(linode_api::resources::account::oauth_clients::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionAccountOauthClientsClientIdApiVersionEnum::V4,
            client_id: "string".to_string(),
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
        .oauth_clients()
        .list(linode_api::resources::account::oauth_clients::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountOauthClientsApiVersionEnum::V4,
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
        .oauth_clients()
        .get(linode_api::resources::account::oauth_clients::GetRequest {
            api_version: linode_api::models::GetApiVersionAccountOauthClientsClientIdApiVersionEnum::V4,
            client_id: "string".to_string(),
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
        .oauth_clients()
        .create(linode_api::resources::account::oauth_clients::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountOauthClientsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountOauthClientsBody {
                id: Some("2737bf16b39ab5d7b4a1".to_string()),
                label: Some("Test_Client_1".to_string()),
                public: Some(false),
                redirect_uri: Some("https://example.org/oauth/callback".to_string()),
                secret: Some("<REDACTED>".to_string()),
                status: Some(
                    linode_api::models::PostApiVersionAccountOauthClientsBodyStatusEnum::Active,
                ),
                thumbnail_url: linode_api::Patch::new(
                    "https://api.linode.com/v4/account/clients/2737bf16b39ab5d7b4a1/thumbnail"
                        .to_string(),
                ),
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
        .account()
        .oauth_clients()
        .put(linode_api::resources::account::oauth_clients::PutRequest {
            api_version: linode_api::models::PutApiVersionAccountOauthClientsClientIdApiVersionEnum::V4,
            client_id: "string".to_string(),
            data: linode_api::models::PutApiVersionAccountOauthClientsClientIdBody {
                id: Some("2737bf16b39ab5d7b4a1".to_string()),
                label: Some("Test_Client_1".to_string()),
                public: Some(false),
                redirect_uri: Some("https://example.org/oauth/callback".to_string()),
                secret: Some("<REDACTED>".to_string()),
                status: Some(
                    linode_api::models::PutApiVersionAccountOauthClientsClientIdBodyStatusEnum::Active,
                ),
                thumbnail_url: linode_api::Patch::new(
                    "https://api.linode.com/v4/account/clients/2737bf16b39ab5d7b4a1/thumbnail"
                        .to_string(),
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
