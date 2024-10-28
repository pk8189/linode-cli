#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .managed()
        .credentials()
        .sshkey()
        .list(linode_api::resources::managed::credentials::sshkey::ListRequest {
            api_version: linode_api::models::GetApiVersionManagedCredentialsSshkeyApiVersionEnum::V4,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
