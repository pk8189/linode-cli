#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .longview()
        .subscriptions()
        .list(linode_api::resources::longview::subscriptions::ListRequest {
            api_version: linode_api::models::GetApiVersionLongviewSubscriptionsApiVersionEnum::V4,
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
        .longview()
        .subscriptions()
        .get(linode_api::resources::longview::subscriptions::GetRequest {
            api_version: linode_api::models::GetApiVersionLongviewSubscriptionsSubscriptionIdApiVersionEnum::V4,
            subscription_id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
