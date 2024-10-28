#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .lke()
        .clusters()
        .api_endpoints()
        .list(linode_api::resources::lke::clusters::api_endpoints::ListRequest {
            api_version: linode_api::models::GetApiVersionLkeClustersClusterIdApiEndpointsApiVersionEnum::V4,
            cluster_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
