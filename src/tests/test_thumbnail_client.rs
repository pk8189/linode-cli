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
        .thumbnail()
        .list(linode_api::resources::account::oauth_clients::thumbnail::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountOauthClientsClientIdThumbnailApiVersionEnum::V4,
            client_id: "string".to_string(),
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
        .thumbnail()
        .put(linode_api::resources::account::oauth_clients::thumbnail::PutRequest {
            api_version: linode_api::models::PutApiVersionAccountOauthClientsClientIdThumbnailApiVersionEnum::V4,
            client_id: "string".to_string(),
            data: linode_api::UploadFile::from_path("uploads/file.pdf").unwrap(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
