#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .clusters()
        .list(linode_api::resources::object_storage::clusters::ListRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageClustersApiVersionEnum::V4,
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
        .object_storage()
        .clusters()
        .get(linode_api::resources::object_storage::clusters::GetRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageClustersClusterIdApiVersionEnum::V4,
            cluster_id: "us-east-1".to_string(),
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
        .delete(linode_api::resources::lke::clusters::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdApiVersionEnum::V4,
            cluster_id: 123,
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
        .list(linode_api::resources::lke::clusters::ListRequest {
            api_version: linode_api::models::GetApiVersionLkeClustersApiVersionEnum::V4,
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
        .get(linode_api::resources::lke::clusters::GetRequest {
            api_version: linode_api::models::GetApiVersionLkeClustersClusterIdApiVersionEnum::V4,
            cluster_id: 123,
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
        .create(linode_api::resources::lke::clusters::CreateRequest {
            api_version: linode_api::models::PostApiVersionLkeClustersApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionLkeClustersBody {
                control_plane: Some(linode_api::models::PostApiVersionLkeClustersBodyControlPlane {
                    acl: Some(linode_api::models::PostApiVersionLkeClustersBodyControlPlaneAcl {
                        addresses: Some(linode_api::models::PostApiVersionLkeClustersBodyControlPlaneAclAddresses {
                            ipv4: Some(
                                vec!["203.0.113.1".to_string(), "192.0.2.0/24".to_string()],
                            ),
                            ipv6: Some(vec!["2001:db8:1234:abcd::/64".to_string()]),
                            ..Default::default()
                        }),
                        enabled: Some(true),
                        revision_id: Some("20240127r001".to_string()),
                        ..Default::default()
                    }),
                    high_availability: Some(true),
                    ..Default::default()
                }),
                k8s_version: "1.27".to_string(),
                label: "lkecluster12345".to_string(),
                node_pools: vec![
                    linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItem {
                    autoscaler :
                    Some(linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemAutoscaler
                    { enabled : Some(true), max : Some(12), min : Some(3),
                    ..Default::default() }), count : 6, disks :
                    Some(vec![linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemDisksItem
                    { size : Some(1024), type_field :
                    Some(linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemDisksItemTypeEnum::Ext4),
                    ..Default::default() }]), labels :
                    Some(linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemLabels
                    { key : Some("example.com/my-app".to_string()), value : Some("teams"
                    .to_string()), ..Default::default() }), tags :
                    Some(vec!["example tag".to_string(), "another example".to_string()]),
                    taints :
                    Some(vec![linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItem
                    { effect :
                    linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItemEffectEnum::NoSchedule,
                    key : "example.com/my-app".to_string(), value : "teamA".to_string(),
                    ..Default::default() },
                    linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItem
                    { effect :
                    linode_api::models::PostApiVersionLkeClustersBodyNodePoolsItemTaintsItemEffectEnum::NoExecute,
                    key : "myapp.io/team".to_string(), value : "teamB".to_string(),
                    ..Default::default() }]), type_field : "g6-standard-4".to_string(),
                    ..Default::default() }
                ],
                region: "us-central".to_string(),
                tags: Some(vec!["ecomm".to_string(), "blogs".to_string()]),
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
        .put(linode_api::resources::lke::clusters::PutRequest {
            api_version: linode_api::models::PutApiVersionLkeClustersClusterIdApiVersionEnum::V4,
            cluster_id: 123,
            data: linode_api::models::PutApiVersionLkeClustersClusterIdBody {
                control_plane: Some(linode_api::models::PutApiVersionLkeClustersClusterIdBodyControlPlane {
                    acl: Some(linode_api::models::PutApiVersionLkeClustersClusterIdBodyControlPlaneAcl {
                        addresses: Some(linode_api::models::PutApiVersionLkeClustersClusterIdBodyControlPlaneAclAddresses {
                            ipv4: Some(
                                vec!["203.0.113.1".to_string(), "192.0.2.0/24".to_string()],
                            ),
                            ipv6: Some(vec!["2001:db8:1234:abcd::/64".to_string()]),
                            ..Default::default()
                        }),
                        enabled: Some(true),
                        revision_id: Some("20240127r001".to_string()),
                        ..Default::default()
                    }),
                    high_availability: Some(true),
                    ..Default::default()
                }),
                k8s_version: Some("string".to_string()),
                label: Some("lkecluster12345".to_string()),
                tags: Some(
                    vec![
                        "prod".to_string(), "monitoring".to_string(), "ecomm"
                        .to_string(), "blog".to_string()
                    ],
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
