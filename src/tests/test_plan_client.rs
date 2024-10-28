#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .longview()
        .plan()
        .list(linode_api::resources::longview::plan::ListRequest {
            api_version: linode_api::models::GetApiVersionLongviewPlanApiVersionEnum::V4,
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
        .longview()
        .plan()
        .put(linode_api::resources::longview::plan::PutRequest {
            api_version: linode_api::models::PutApiVersionLongviewPlanApiVersionEnum::V4,
            data: linode_api::models::PutApiVersionLongviewPlanBody {
                longview_subscription: linode_api::Patch::new(
                    linode_api::models::PutApiVersionLongviewPlanBodyLongviewSubscriptionEnum::Longview10,
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
