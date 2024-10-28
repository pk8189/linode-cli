#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .lke()
        .clusters()
        .servicetoken()
        .delete(linode_api::resources::lke::clusters::servicetoken::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdServicetokenApiVersionEnum::V4,
            cluster_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
