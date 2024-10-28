#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .lke()
        .clusters()
        .recycle()
        .create(linode_api::resources::lke::clusters::recycle::CreateRequest {
            api_version: linode_api::models::PostApiVersionLkeClustersClusterIdRecycleApiVersionEnum::V4,
            cluster_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .lke()
        .clusters()
        .pools()
        .recycle()
        .create(linode_api::resources::lke::clusters::pools::recycle::CreateRequest {
            api_version: linode_api::models::PostApiVersionLkeClustersClusterIdPoolsPoolIdRecycleApiVersionEnum::V4,
            cluster_id: 123,
            pool_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .lke()
        .clusters()
        .nodes()
        .recycle()
        .create(linode_api::resources::lke::clusters::nodes::recycle::CreateRequest {
            api_version: linode_api::models::PostApiVersionLkeClustersClusterIdNodesNodeIdRecycleApiVersionEnum::V4,
            cluster_id: 123,
            node_id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
