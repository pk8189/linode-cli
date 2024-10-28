#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .images()
        .upload()
        .create(linode_api::resources::images::upload::CreateRequest {
            api_version: linode_api::models::PostApiVersionImagesUploadApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionImagesUploadBody {
                cloud_init: Some(true),
                description: Some("This is an example image in the docs.".to_string()),
                label: "my-image-label".to_string(),
                region: "eu-central".to_string(),
                tags: Some(vec!["repair-image".to_string(), "fix-1".to_string()]),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
