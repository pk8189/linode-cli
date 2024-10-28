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
        .control_plane_acl()
        .delete(linode_api::resources::lke::clusters::control_plane_acl::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLkeClustersClusterIdControlPlaneAclApiVersionEnum::V4,
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
        .control_plane_acl()
        .list(linode_api::resources::lke::clusters::control_plane_acl::ListRequest {
            api_version: linode_api::models::GetApiVersionLkeClustersClusterIdControlPlaneAclApiVersionEnum::V4,
            cluster_id: 123,
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
        .control_plane_acl()
        .put(linode_api::resources::lke::clusters::control_plane_acl::PutRequest {
            api_version: linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclApiVersionEnum::V4,
            cluster_id: 123,
            data: linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBody {
                acl: Some(linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAcl {
                    addresses: Some(linode_api::models::PutApiVersionLkeClustersClusterIdControlPlaneAclBodyAclAddresses {
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
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
