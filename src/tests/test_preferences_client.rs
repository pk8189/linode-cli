#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .preferences()
        .list(linode_api::resources::profile_resource::preferences::ListRequest {
            api_version: linode_api::models::GetApiVersionProfilePreferencesApiVersionEnum::V4,
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
        .preferences()
        .put(linode_api::resources::profile_resource::preferences::PutRequest {
            api_version: linode_api::models::PutApiVersionProfilePreferencesApiVersionEnum::V4,
            data: serde_json::json!({}),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
