#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .networking()
        .ipv6()
        .pools()
        .list(linode_api::resources::networking::ipv6::pools::ListRequest {
            api_version: linode_api::models::GetApiVersionNetworkingIpv6PoolsApiVersionEnum::V4,
            ..Default::default()
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
        .pools()
        .delete(linode_api::resources::lke::clusters::pools::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdPoolsPoolIdApiVersionEnum::V4,
            cluster_id: 123,
            pool_id: 123,
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
        .lke()
        .clusters()
        .pools()
        .list(linode_api::resources::lke::clusters::pools::ListRequest {
            api_version: linode_api::models::GetApiVersionLkeClustersClusterIdPoolsApiVersionEnum::V4,
            cluster_id: 123,
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
        .pools()
        .get(linode_api::resources::lke::clusters::pools::GetRequest {
            api_version: linode_api::models::GetApiVersionLkeClustersClusterIdPoolsPoolIdApiVersionEnum::V4,
            cluster_id: 123,
            pool_id: 123,
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
        .lke()
        .clusters()
        .pools()
        .create(linode_api::resources::lke::clusters::pools::CreateRequest {
            api_version: linode_api::models::PostApiVersionLkeClustersClusterIdPoolsApiVersionEnum::V4,
            cluster_id: 123,
            data: linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBody {
                autoscaler: Some(linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyAutoscaler {
                    enabled: Some(true),
                    max: Some(12),
                    min: Some(3),
                    ..Default::default()
                }),
                count: 6,
                disks: Some(
                    vec![
                        linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyDisksItem
                        { size : Some(1024), type_field :
                        Some(linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyDisksItemTypeEnum::Ext4),
                        ..Default::default() }
                    ],
                ),
                labels: Some(linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyLabels {
                    key: Some("example.com/my-app".to_string()),
                    value: Some("teams".to_string()),
                    ..Default::default()
                }),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
                taints: Some(
                    vec![
                        linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItem
                        { effect :
                        linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItemEffectEnum::NoSchedule,
                        key : "example.com/my-app".to_string(), value : "teamA"
                        .to_string(), ..Default::default() },
                        linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItem
                        { effect :
                        linode_api::models::PostApiVersionLkeClustersClusterIdPoolsBodyTaintsItemEffectEnum::NoExecute,
                        key : "myapp.io/team".to_string(), value : "teamB".to_string(),
                        ..Default::default() }
                    ],
                ),
                type_field: "g6-standard-4".to_string(),
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
        .lke()
        .clusters()
        .pools()
        .put(linode_api::resources::lke::clusters::pools::PutRequest {
            api_version: linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdApiVersionEnum::V4,
            cluster_id: 123,
            pool_id: 123,
            data: linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBody {
                autoscaler: Some(linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyAutoscaler {
                    enabled: Some(true),
                    max: Some(12),
                    min: Some(3),
                    ..Default::default()
                }),
                count: Some(6),
                labels: Some(linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyLabels {
                    key: Some("example.com/my-app".to_string()),
                    value: Some("teams".to_string()),
                    ..Default::default()
                }),
                taints: Some(
                    vec![
                        linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItem
                        { effect :
                        linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItemEffectEnum::NoSchedule,
                        key : "example.com/my-app".to_string(), value : "teamA"
                        .to_string(), ..Default::default() },
                        linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItem
                        { effect :
                        linode_api::models::PutApiVersionLkeClustersClusterIdPoolsPoolIdBodyTaintsItemEffectEnum::NoExecute,
                        key : "myapp.io/team".to_string(), value : "teamB".to_string(),
                        ..Default::default() }
                    ],
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
