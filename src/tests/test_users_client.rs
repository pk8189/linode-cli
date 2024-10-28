#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .users()
        .delete(linode_api::resources::account::users::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionAccountUsersUsernameApiVersionEnum::V4,
            username: "string".to_string(),
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
        .users()
        .list(linode_api::resources::account::users::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountUsersApiVersionEnum::V4,
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
        .users()
        .get(linode_api::resources::account::users::GetRequest {
            api_version: linode_api::models::GetApiVersionAccountUsersUsernameApiVersionEnum::V4,
            username: "string".to_string(),
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
        .users()
        .create(linode_api::resources::account::users::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountUsersApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountUsersBody {
                email: Some("example_user@linode.com".to_string()),
                last_login: linode_api::Patch::new(linode_api::models::PostApiVersionAccountUsersBodyLastLogin {
                    login_datetime: Some("2018-01-01T01:01:01".to_string()),
                    status: Some(
                        linode_api::models::PostApiVersionAccountUsersBodyLastLoginStatusEnum::Successful,
                    ),
                    ..Default::default()
                }),
                password_created: linode_api::Patch::new(
                    "2018-01-01T01:01:01".to_string(),
                ),
                restricted: Some(true),
                ssh_keys: Some(vec!["home-pc".to_string(), "laptop".to_string()]),
                tfa_enabled: Some(true),
                username: Some("example_user".to_string()),
                verified_phone_number: linode_api::Patch::new("+5555555555".to_string()),
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
        .users()
        .put(linode_api::resources::account::users::PutRequest {
            api_version: linode_api::models::PutApiVersionAccountUsersUsernameApiVersionEnum::V4,
            username: "string".to_string(),
            data: linode_api::models::PutApiVersionAccountUsersUsernameBody {
                email: Some("example_user@linode.com".to_string()),
                last_login: linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameBodyLastLogin {
                    login_datetime: Some("2018-01-01T01:01:01".to_string()),
                    status: Some(
                        linode_api::models::PutApiVersionAccountUsersUsernameBodyLastLoginStatusEnum::Successful,
                    ),
                    ..Default::default()
                }),
                password_created: linode_api::Patch::new(
                    "2018-01-01T01:01:01".to_string(),
                ),
                restricted: Some(true),
                ssh_keys: Some(vec!["home-pc".to_string(), "laptop".to_string()]),
                tfa_enabled: Some(true),
                username: Some("example_user".to_string()),
                verified_phone_number: linode_api::Patch::new("+5555555555".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
