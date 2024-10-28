#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .tokens()
        .delete(linode_api::resources::profile_resource::tokens::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionProfileTokensTokenIdApiVersionEnum::V4,
            token_id: 123,
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
        .profile_resource()
        .tokens()
        .list(linode_api::resources::profile_resource::tokens::ListRequest {
            api_version: linode_api::models::GetApiVersionProfileTokensApiVersionEnum::V4,
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
        .profile_resource()
        .tokens()
        .get(linode_api::resources::profile_resource::tokens::GetRequest {
            api_version: linode_api::models::GetApiVersionProfileTokensTokenIdApiVersionEnum::V4,
            token_id: 123,
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
        .profile_resource()
        .tokens()
        .create(linode_api::resources::profile_resource::tokens::CreateRequest {
            api_version: linode_api::models::PostApiVersionProfileTokensApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionProfileTokensBody {
                expiry: Some("1970-01-01T00:00:00".to_string()),
                label: Some("linode-cli".to_string()),
                scopes: Some("*".to_string()),
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
        .profile_resource()
        .tokens()
        .put(linode_api::resources::profile_resource::tokens::PutRequest {
            api_version: linode_api::models::PutApiVersionProfileTokensTokenIdApiVersionEnum::V4,
            token_id: 123,
            data: linode_api::models::PutApiVersionProfileTokensTokenIdBody {
                created: Some("2018-01-01T00:01:01".to_string()),
                expiry: Some("2018-01-01T13:46:32".to_string()),
                id: Some(123),
                label: Some("linode-cli".to_string()),
                scopes: Some("*".to_string()),
                token: Some("abcdefghijklmnop".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
