#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .sshkeys()
        .delete(linode_api::resources::profile_resource::sshkeys::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionProfileSshkeysSshKeyIdApiVersionEnum::V4,
            ssh_key_id: 123,
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
        .sshkeys()
        .list(linode_api::resources::profile_resource::sshkeys::ListRequest {
            api_version: linode_api::models::GetApiVersionProfileSshkeysApiVersionEnum::V4,
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
        .profile_resource()
        .sshkeys()
        .get(linode_api::resources::profile_resource::sshkeys::GetRequest {
            api_version: linode_api::models::GetApiVersionProfileSshkeysSshKeyIdApiVersionEnum::V4,
            ssh_key_id: 123,
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
        .sshkeys()
        .create(linode_api::resources::profile_resource::sshkeys::CreateRequest {
            api_version: linode_api::models::PostApiVersionProfileSshkeysApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionProfileSshkeysBody {
                created: Some("2018-01-01T00:01:01".to_string()),
                id: Some(42),
                label: Some("My SSH Key".to_string()),
                ssh_key: Some(
                    "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer"
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
        .profile_resource()
        .sshkeys()
        .put(linode_api::resources::profile_resource::sshkeys::PutRequest {
            api_version: linode_api::models::PutApiVersionProfileSshkeysSshKeyIdApiVersionEnum::V4,
            ssh_key_id: 123,
            data: linode_api::models::PutApiVersionProfileSshkeysSshKeyIdBody {
                label: Some("My SSH Key".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
