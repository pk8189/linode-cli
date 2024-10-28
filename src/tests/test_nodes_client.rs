#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .nodebalancers()
        .configs()
        .nodes()
        .delete(linode_api::resources::nodebalancers::configs::nodes::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
            node_id: "string".to_string(),
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
        .nodebalancers()
        .configs()
        .nodes()
        .list(linode_api::resources::nodebalancers::configs::nodes::ListRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
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
        .nodebalancers()
        .configs()
        .nodes()
        .get(linode_api::resources::nodebalancers::configs::nodes::GetRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
            node_id: "string".to_string(),
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
        .nodebalancers()
        .configs()
        .nodes()
        .create(linode_api::resources::nodebalancers::configs::nodes::CreateRequest {
            api_version: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
            data: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesBody {
                address: Some("192.168.210.120:80".to_string()),
                config_id: Some(4567),
                id: Some(54321),
                label: Some("node54321".to_string()),
                mode: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesBodyModeEnum::Accept,
                ),
                nodebalancer_id: Some(12345),
                status: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesBodyStatusEnum::Up,
                ),
                weight: Some(50),
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
        .nodebalancers()
        .configs()
        .nodes()
        .put(linode_api::resources::nodebalancers::configs::nodes::PutRequest {
            api_version: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
            node_id: "string".to_string(),
            data: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdBody {
                address: Some("192.168.210.120:80".to_string()),
                config_id: Some(4567),
                id: Some(54321),
                label: Some("node54321".to_string()),
                mode: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdBodyModeEnum::Accept,
                ),
                nodebalancer_id: Some(12345),
                status: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesNodeIdBodyStatusEnum::Up,
                ),
                weight: Some(50),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
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
        .nodes()
        .delete(linode_api::resources::lke::clusters::nodes::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdNodesNodeIdApiVersionEnum::V4,
            cluster_id: 123,
            node_id: "string".to_string(),
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
        .lke()
        .clusters()
        .nodes()
        .get(linode_api::resources::lke::clusters::nodes::GetRequest {
            api_version: linode_api::models::GetApiVersionLkeClustersClusterIdNodesNodeIdApiVersionEnum::V4,
            cluster_id: 123,
            node_id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
